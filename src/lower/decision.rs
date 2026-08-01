//! Compiles matches to decision trees.
//!
//! A match is a matrix: one row per rule, one column per value being
//! matched. The compiler picks a column the first remaining row needs to
//! look at, tests the value there once for each constructor the column
//! mentions, and continues with the rows that agree with the outcome, so no
//! value is tested twice on a path. Rules keep their order, so the first rule
//! that matches wins. Each rule's body is lowered once, in a block whose
//! parameters are the rule's variables; every leaf that selects the rule
//! jumps there.

use super::{Dest, Lowerer, Res, unsupported_pattern};
use crate::core::{self, Atom, BlockId, Op, Prim, Term};
use crate::infer::Ty;
use crate::parser::{Expr, Pat, PatKind};
use crate::value;

/// A pattern, with syntax that only names something else resolved.
#[derive(Clone, Debug)]
enum Pattern {
    /// Matches anything: `_`, `()` and the parts a flexible record omits.
    Any,
    /// Binds the name to the value, which must also match the pattern.
    Bind(String, Box<Pattern>),
    /// A tuple or record, fields in their layout order.
    Product(Vec<Pattern>),
    /// A constructor or a constant.
    Test(Test, Option<Box<Pattern>>),
}

/// A test on one value.
#[derive(Clone, Debug, PartialEq)]
pub enum Test {
    /// The value is this word: an `int`, `word` or `char` constant, or a
    /// nullary constructor. `span` counts the type's values when they are
    /// all constructors, so a switch knows when it has seen them all.
    Word { word: i64, span: Option<usize> },
    /// A string constant.
    String(String),
    /// A constructor with an argument, stored in a heap block. `tag` is the
    /// block's field 0 when the datatype has several such constructors, and
    /// `mixed` says whether it also has nullary ones, which are immediates.
    Boxed {
        tag: Option<i64>,
        mixed: bool,
        span: usize,
    },
    /// `::`: a cons cell, laid out as the pair of its head and tail.
    Cons,
    /// `ref`, which always matches.
    Ref,
}

impl Test {
    /// How many constructors the test's type has, when it is finite.
    fn span(&self) -> Option<usize> {
        match self {
            Test::Word { span, .. } => *span,
            Test::String(_) => None,
            Test::Boxed { span, .. } => Some(*span),
            Test::Cons => Some(2),
            Test::Ref => Some(1),
        }
    }
}

/// A row of the matrix.
#[derive(Clone)]
struct Row {
    patterns: Vec<Pattern>,
    /// Variables bound so far, to the values they name.
    binds: Vec<(String, Atom)>,
    rule: usize,
}

/// What a match jumps to.
struct Targets {
    /// Each rule's block and the names of its parameters.
    rules: Vec<(BlockId, Vec<String>)>,
    /// Whether each rule is selected by some leaf.
    reached: Vec<bool>,
    failure: core::Failure,
    location: String,
}

impl Lowerer<'_> {
    /// Matches `scrutinees` against each rule's patterns in turn and sends
    /// the first matching rule's body to `dest`; no match raises `Match`.
    pub(super) fn rules(
        &mut self,
        scrutinees: &[Atom],
        rules: &[(Vec<&Pat>, &Expr)],
        dest: Dest,
        location: &str,
    ) -> Res<()> {
        let patterns: Vec<Vec<&Pat>> = rules.iter().map(|(patterns, _)| patterns.clone()).collect();
        let blocks = self.decide(scrutinees, &patterns, core::Failure::Match, location)?;
        for ((block, names), (_, body)) in blocks.into_iter().zip(rules) {
            let Some(block) = block else { continue };
            self.switch(block);
            let mark = self.scope.len();
            let params = self.frame().blocks[block].0.clone();
            for (name, var) in names.iter().zip(params) {
                self.bind_local(name, Atom::Var(var));
            }
            self.into(body, dest)?;
            self.scope.truncate(mark);
        }
        Ok(())
    }

    /// Compiles the match of `scrutinees` against the rows `patterns`,
    /// ending the current block. Returns, for each rule some value can
    /// select, the block entered when it does and the names its parameters
    /// bind; when no rule matches, `failure` is raised at `location`.
    pub(super) fn decide(
        &mut self,
        scrutinees: &[Atom],
        patterns: &[Vec<&Pat>],
        failure: core::Failure,
        location: &str,
    ) -> Res<Vec<(Option<BlockId>, Vec<String>)>> {
        let mut rows = Vec::new();
        let mut targets = Targets {
            rules: Vec::new(),
            reached: vec![false; patterns.len()],
            failure,
            location: location.to_string(),
        };
        for (rule, row) in patterns.iter().enumerate() {
            let mut names = Vec::new();
            let mut resolved = Vec::new();
            for pattern in row {
                let pattern = self.resolve_pattern(pattern)?;
                variables(&pattern, &mut names);
                resolved.push(pattern);
            }
            let params = names.iter().map(|name| self.frame().var(name)).collect();
            let block = self.block(params);
            targets.rules.push((block, names));
            rows.push(Row {
                patterns: resolved,
                binds: Vec::new(),
                rule,
            });
        }
        self.tree(scrutinees.to_vec(), rows, &mut targets);
        Ok(targets
            .rules
            .into_iter()
            .zip(targets.reached)
            .map(|((block, names), reached)| (reached.then_some(block), names))
            .collect())
    }

    /// Emits the decision tree for `rows` over the values `occurrences`.
    fn tree(&mut self, occurrences: Vec<Atom>, mut rows: Vec<Row>, targets: &mut Targets) {
        if rows.is_empty() {
            self.terminate(Term::Fail(targets.failure, targets.location.clone()));
            return;
        }
        // Bindings name the value in their column, whatever else it matches.
        for row in &mut rows {
            for (pattern, occurrence) in row.patterns.iter_mut().zip(&occurrences) {
                while let Pattern::Bind(name, inner) = pattern {
                    row.binds.push((name.clone(), occurrence.clone()));
                    *pattern = std::mem::replace(inner.as_mut(), Pattern::Any);
                }
            }
        }
        let Some(column) = rows[0]
            .patterns
            .iter()
            .position(|pattern| !matches!(pattern, Pattern::Any))
        else {
            // The first row matches.
            let row = &rows[0];
            let (block, names) = &targets.rules[row.rule];
            let args = names
                .iter()
                .map(|name| {
                    row.binds
                        .iter()
                        .rev()
                        .find(|(bound, _)| bound == name)
                        .map(|(_, atom)| atom.clone())
                        .expect("every variable of a matched row is bound")
                })
                .collect();
            targets.reached[row.rule] = true;
            self.terminate(Term::Jump(*block, args));
            return;
        };
        match &rows[0].patterns[column] {
            Pattern::Product(fields) => {
                let width = fields.len();
                let value = occurrences[column].clone();
                let loaded: Vec<Atom> = (0..width)
                    .map(|index| self.bind("", Op::Select(value.clone(), index)))
                    .collect();
                let occurrences = splice(&occurrences, column, loaded);
                let rows = rows
                    .into_iter()
                    .map(|mut row| {
                        let fields =
                            match std::mem::replace(&mut row.patterns[column], Pattern::Any) {
                                Pattern::Product(fields) => fields,
                                _ => vec![Pattern::Any; width],
                            };
                        row.patterns = splice(&row.patterns, column, fields);
                        row
                    })
                    .collect();
                self.tree(occurrences, rows, targets);
            }
            Pattern::Test(..) => self.switch_on(column, occurrences, rows, targets),
            Pattern::Any | Pattern::Bind(..) => unreachable!("the column needs a test"),
        }
    }

    /// Tests the value in `column` against each constructor the column
    /// mentions, in the order the rows mention them.
    fn switch_on(
        &mut self,
        column: usize,
        occurrences: Vec<Atom>,
        rows: Vec<Row>,
        targets: &mut Targets,
    ) {
        let mut tests: Vec<Test> = Vec::new();
        for row in &rows {
            if let Pattern::Test(test, _) = &row.patterns[column]
                && !tests.contains(test)
            {
                tests.push(test.clone());
            }
        }
        let complete = tests[0].span() == Some(tests.len());
        let value = occurrences[column].clone();
        for (index, test) in tests.iter().enumerate() {
            let last = index + 1 == tests.len();
            let otherwise = if complete && last {
                None
            } else {
                let condition = self.condition(test, value.clone());
                let then = self.block(Vec::new());
                let otherwise = self.block(Vec::new());
                self.terminate(Term::If(condition, then, otherwise));
                self.switch(then);
                Some(otherwise)
            };
            // The rows that can still match once the test passed.
            let argument = self.argument(test, value.clone());
            let specialised = rows
                .iter()
                .filter_map(|row| {
                    let inner = match &row.patterns[column] {
                        Pattern::Test(found, inner) if found == test => inner.as_deref().cloned(),
                        Pattern::Test(..) => return None,
                        _ => None,
                    };
                    let mut row = row.clone();
                    let replacement = match (&argument, inner) {
                        (Some(_), inner) => vec![inner.unwrap_or(Pattern::Any)],
                        (None, _) => Vec::new(),
                    };
                    row.patterns = splice(&row.patterns, column, replacement);
                    Some(row)
                })
                .collect();
            let occurrences = splice(&occurrences, column, argument.into_iter().collect());
            self.tree(occurrences, specialised, targets);
            match otherwise {
                Some(otherwise) => self.switch(otherwise),
                None => return,
            }
        }
        // No constructor the rows mention: only rows that match anything here
        // are left.
        let rows = rows
            .into_iter()
            .filter(|row| !matches!(row.patterns[column], Pattern::Test(..)))
            .map(|mut row| {
                row.patterns.remove(column);
                row
            })
            .collect();
        let mut occurrences = occurrences;
        occurrences.remove(column);
        self.tree(occurrences, rows, targets);
    }

    /// Whether `value` passes `test`.
    fn condition(&mut self, test: &Test, value: Atom) -> Atom {
        match test {
            Test::Word { word, .. } => {
                self.bind("", Op::Prim(Prim::WordEq, vec![value, Atom::Word(*word)]))
            }
            Test::String(text) => self.bind(
                "",
                Op::Prim(Prim::Equal, vec![value, Atom::String(text.clone())]),
            ),
            Test::Cons | Test::Boxed { tag: None, .. } => {
                self.bind("", Op::Prim(Prim::IsBoxed, vec![value]))
            }
            Test::Boxed {
                tag: Some(tag),
                mixed,
                ..
            } => {
                if *mixed {
                    // Nullary constructors are immediates, which have no tag
                    // to load: test for a block first.
                    let result = self.frame().var("");
                    let join = self.block(vec![result]);
                    let boxed = self.bind("", Op::Prim(Prim::IsBoxed, vec![value.clone()]));
                    let load = self.block(Vec::new());
                    let no = self.block(Vec::new());
                    self.terminate(Term::If(boxed, load, no));
                    self.switch(no);
                    self.terminate(Term::Jump(join, vec![Atom::Word(value::FALSE)]));
                    self.switch(load);
                    let found = self.bind("", Op::Select(value, 0));
                    let equal =
                        self.bind("", Op::Prim(Prim::WordEq, vec![found, Atom::Word(*tag)]));
                    self.terminate(Term::Jump(join, vec![equal]));
                    self.switch(join);
                    Atom::Var(result)
                } else {
                    let found = self.bind("", Op::Select(value, 0));
                    self.bind("", Op::Prim(Prim::WordEq, vec![found, Atom::Word(*tag)]))
                }
            }
            Test::Ref => Atom::Word(value::TRUE),
        }
    }

    /// The argument of the constructor `test`, once `value` is known to be
    /// built by it; `None` for constants.
    fn argument(&mut self, test: &Test, value: Atom) -> Option<Atom> {
        match test {
            Test::Word { .. } | Test::String(_) => None,
            // A cons cell is its own argument: the pair of head and tail.
            Test::Cons => Some(value),
            Test::Ref | Test::Boxed { tag: None, .. } => Some(self.bind("", Op::Select(value, 0))),
            Test::Boxed { tag: Some(_), .. } => Some(self.bind("", Op::Select(value, 1))),
        }
    }

    /// Resolves the syntax of `pattern`.
    fn resolve_pattern(&self, pattern: &Pat) -> Res<Pattern> {
        Ok(match &pattern.value {
            PatKind::Wildcard | PatKind::Unit => Pattern::Any,
            PatKind::Variable(name)
                if self.lookup(name).is_none()
                    && self.types.pat(pattern).is_some_and(|ty| ty.is("exn"))
                    && BUILTIN_EXCEPTIONS.contains(&name.as_str()) =>
            {
                return Err(unsupported_pattern("exception patterns", pattern));
            }
            PatKind::Variable(name) => match self.constructor(name, self.types.pat(pattern)) {
                Some((test, false)) => Pattern::Test(test, None),
                Some((_, true)) => return Err(unsupported_pattern("this constructor", pattern)),
                None => Pattern::Bind(name.clone(), Box::new(Pattern::Any)),
            },
            PatKind::Integer(integer) => word(value::tagged(*integer)),
            PatKind::Character(character) => word(value::tagged(i64::from(u32::from(*character)))),
            PatKind::Word(text) => match parse_word(text) {
                Some(parsed) => word(value::tagged(parsed)),
                None => return Err(unsupported_pattern("this word constant", pattern)),
            },
            PatKind::Boolean(boolean) => Pattern::Test(
                Test::Word {
                    word: value::tagged(i64::from(*boolean)),
                    span: Some(2),
                },
                None,
            ),
            PatKind::String(text) => Pattern::Test(Test::String(text.clone()), None),
            PatKind::Tuple(items) => Pattern::Product(
                items
                    .iter()
                    .map(|item| self.resolve_pattern(item))
                    .collect::<Res<_>>()?,
            ),
            PatKind::Record(fields, _) => {
                let Some(Ty::Record(labels)) = self.types.pat(pattern) else {
                    return Err(unsupported_pattern("this record pattern", pattern));
                };
                // Fields a flexible record leaves out match anything.
                let mut resolved = vec![Pattern::Any; labels.len()];
                for (label, item) in fields {
                    let index = labels
                        .iter()
                        .position(|(known, _)| known == label)
                        .expect("the record type has the pattern's labels");
                    resolved[index] = self.resolve_pattern(item)?;
                }
                Pattern::Product(resolved)
            }
            PatKind::List(items) => {
                let mut list = Pattern::Test(
                    Test::Word {
                        word: value::NIL,
                        span: Some(2),
                    },
                    None,
                );
                for item in items.iter().rev() {
                    let head = self.resolve_pattern(item)?;
                    list = Pattern::Test(
                        Test::Cons,
                        Some(Box::new(Pattern::Product(vec![head, list]))),
                    );
                }
                list
            }
            PatKind::Cons(head, tail) => Pattern::Test(
                Test::Cons,
                Some(Box::new(Pattern::Product(vec![
                    self.resolve_pattern(head)?,
                    self.resolve_pattern(tail)?,
                ]))),
            ),
            PatKind::Constructor(name, argument) => {
                match self.constructor(name, self.types.pat(pattern)) {
                    Some((test, true)) => {
                        Pattern::Test(test, Some(Box::new(self.resolve_pattern(argument)?)))
                    }
                    _ => return Err(unsupported_pattern("this constructor", pattern)),
                }
            }
            PatKind::Layered(name, _, inner) => {
                Pattern::Bind(name.clone(), Box::new(self.resolve_pattern(inner)?))
            }
            PatKind::Typed(inner, _) => self.resolve_pattern(inner)?,
        })
    }

    /// The test for the constructor `name` and whether it takes an argument,
    /// or `None` if `name` is a variable. `ty` is the type of the pattern or
    /// expression that names it: the datatype, or a function returning it.
    ///
    /// A datatype's nullary constructors are the immediates `0`, `1`, ... in
    /// declaration order. Its constructors with an argument are blocks: the
    /// argument alone when there is one such constructor, else a tag
    /// `0`, `1`, ... and then the argument.
    pub(super) fn constructor(&self, name: &str, ty: Option<&Ty>) -> Option<(Test, bool)> {
        let base = name.rsplit('.').next().unwrap_or(name);
        let result = match ty {
            Some(Ty::Arrow(_, result)) => Some(result.as_ref()),
            ty => ty,
        };
        if let Some(Ty::Con { stamp, .. }) = result
            && *stamp != 0
            && let Some(constructors) = self.types.constructors(*stamp)
        {
            let position = constructors.iter().position(|(known, _)| known == base)?;
            let carries = constructors[position].1;
            let span = constructors.len();
            let before = constructors[..position]
                .iter()
                .filter(|(_, other)| *other == carries)
                .count();
            let index = i64::try_from(before).expect("a small index");
            let carriers = constructors.iter().filter(|(_, other)| *other).count();
            let test = if carries {
                Test::Boxed {
                    tag: (carriers > 1).then_some(value::tagged(index)),
                    mixed: carriers < span,
                    span,
                }
            } else {
                Test::Word {
                    word: value::tagged(index),
                    span: Some(span),
                }
            };
            return Some((test, carries));
        }
        let nullary = |index: i64, span: usize| {
            Some((
                Test::Word {
                    word: value::tagged(index),
                    span: Some(span),
                },
                false,
            ))
        };
        if self.lookup(name).is_some() {
            return None;
        }
        match name {
            "nil" => nullary(0, 2),
            "::" => Some((Test::Cons, true)),
            "true" => nullary(1, 2),
            "false" => nullary(0, 2),
            "ref" => Some((Test::Ref, true)),
            "NONE" => nullary(0, 2),
            "SOME" => Some((
                Test::Boxed {
                    tag: None,
                    mixed: true,
                    span: 2,
                },
                true,
            )),
            "LESS" => nullary(0, 3),
            "EQUAL" => nullary(1, 3),
            "GREATER" => nullary(2, 3),
            _ => None,
        }
    }
}

/// The basis's exceptions, which a pattern names as constructors.
const BUILTIN_EXCEPTIONS: &[&str] = &["Div", "Overflow", "Match", "Bind", "Empty", "Subscript"];

fn word(word: i64) -> Pattern {
    Pattern::Test(Test::Word { word, span: None }, None)
}

/// The value of a `word` constant such as `0w5` or `0wxFF`.
pub fn parse_word(text: &str) -> Option<i64> {
    let digits = text.strip_prefix("0w")?;
    match digits.strip_prefix('x') {
        Some(hex) => i64::from_str_radix(hex, 16).ok(),
        None => digits.parse().ok(),
    }
}

/// The variables `pattern` binds, in order.
fn variables(pattern: &Pattern, names: &mut Vec<String>) {
    match pattern {
        Pattern::Any => {}
        Pattern::Bind(name, inner) => {
            names.push(name.clone());
            variables(inner, names);
        }
        Pattern::Product(fields) => fields.iter().for_each(|field| variables(field, names)),
        Pattern::Test(_, argument) => {
            if let Some(argument) = argument {
                variables(argument, names);
            }
        }
    }
}

/// `items` with the element at `index` replaced by `replacement`.
fn splice<T: Clone>(items: &[T], index: usize, replacement: Vec<T>) -> Vec<T> {
    let mut spliced = items[..index].to_vec();
    spliced.extend(replacement);
    spliced.extend_from_slice(&items[index + 1..]);
    spliced
}
