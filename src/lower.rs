//! Lowers a type-checked program to the core IR.
//!
//! Each SML function becomes an IR function whose first parameter is its
//! closure. A function's free variables are found as its body is lowered:
//! the first use of a variable bound by an enclosing function adds it to the
//! closure and loads it from the environment on entry. Top-level bindings
//! live in globals instead, so they are never captured.

use miette::SourceSpan;

use crate::core::{
    Atom, Block, BlockId, Callee, Closure, FnId, Function, GlobalId, Module, Op, Prim, Stmt, Term,
    Var,
};
use crate::error::ThisError;
use crate::infer::{Ty, TypeTable};
use crate::parser::{Decl, DeclKind, Expr, ExprKind, Pat, PatKind, Program, StmtKind};
use crate::value;

#[derive(Debug, ThisError)]
pub enum LowerError {
    #[error("{0} are not supported by code generation yet")]
    Unsupported(String),
}

pub type Failure = (LowerError, SourceSpan);
type Res<T> = Result<T, Failure>;

/// A function whose code is known where it is called: calls with at least
/// `arity` arguments go straight to `worker`, which takes the closure and
/// then `arity` arguments.
#[derive(Clone, Copy, Debug)]
struct Known {
    worker: FnId,
    arity: usize,
}

/// What a name stands for.
#[derive(Clone, Debug)]
enum Binding {
    Global(GlobalId, Option<Known>),
    /// A variable of the function at `depth` in the stack being lowered.
    Local {
        depth: usize,
        var: Var,
        known: Option<Known>,
    },
}

/// What lowering keeps from one program to the next: the REPL lowers each
/// chunk in the environment of the earlier ones.
#[derive(Clone, Default)]
pub struct Session {
    next_function: FnId,
    next_global: GlobalId,
    /// Top-level names, latest last.
    globals: Vec<(String, Binding)>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    /// The global that holds the top-level binding `name`.
    pub fn global(&self, name: &str) -> Option<GlobalId> {
        match self.lookup(name)? {
            Binding::Global(global, _) => Some(global),
            Binding::Local { .. } => None,
        }
    }

    fn lookup(&self, name: &str) -> Option<Binding> {
        self.globals
            .iter()
            .rev()
            .find(|(known, _)| known == name)
            .map(|(_, binding)| binding.clone())
    }

    fn function_id(&mut self) -> FnId {
        self.next_function += 1;
        self.next_function - 1
    }

    /// Lowers `program`, read from `source`; its top-level code becomes the
    /// function `entry`.
    pub fn lower(
        &mut self,
        program: &Program,
        types: &TypeTable,
        source: &Source,
        entry: &str,
    ) -> Res<Module> {
        let mut session = self.clone();
        let id = session.function_id();
        let mut lowerer = Lowerer {
            session: &mut session,
            types,
            source,
            new_globals: Vec::new(),
            functions: Vec::new(),
            frames: vec![Builder::new(id, entry, Vec::new())],
            scope: Vec::new(),
        };
        for statement in &program.statements {
            lowerer.statement(&statement.value)?;
        }
        let status = value::tagged(i64::from(program.result));
        lowerer.terminate(Term::Return(Atom::Word(status)));
        let entry = lowerer.frames.pop().expect("the entry frame").finish();
        let module = Module {
            functions: lowerer.functions,
            globals: lowerer.new_globals,
            entry,
            file: source.file.clone(),
        };
        *self = session;
        Ok(module)
    }
}

/// The program's source, for the positions SML/NJ reports exceptions at.
pub struct Source {
    /// The file name as SML/NJ shows it: without directories.
    pub file: String,
    pub text: String,
}

impl Source {
    /// SML/NJ's `line.column` of the byte at `offset`, both counted from 1.
    fn position(&self, offset: usize) -> String {
        let before = &self.text[..offset.min(self.text.len())];
        let line = before.matches('\n').count() + 1;
        let column = before.len() - before.rfind('\n').map_or(0, |newline| newline + 1) + 1;
        format!("{line}.{column}")
    }

    /// Where SML/NJ reports an exception raised by the infix operator `name`
    /// between `lhs` and `rhs`: the operator's own span.
    fn operator(&self, name: &str, lhs: &Expr, rhs: &Expr) -> String {
        let between = &self.text[lhs.end.offset..rhs.start.offset];
        let start = lhs.end.offset + between.find(name).unwrap_or(0);
        format!(
            "{}:{}-{}",
            self.file,
            self.position(start),
            self.position(start + name.len())
        )
    }

    /// Where a failed match is reported: the end of `expr`.
    fn end(&self, expr: &Expr) -> String {
        format!("{}:{}", self.file, self.position(expr.end.offset))
    }
}

/// A function whose blocks are being built.
struct Builder {
    id: FnId,
    name: String,
    params: Vec<Var>,
    vars: Vec<String>,
    blocks: Vec<(Vec<Var>, Vec<Stmt>, Option<Term>)>,
    current: BlockId,
    /// Loads of captured variables from the environment, run on entry.
    prologue: Vec<Stmt>,
    /// The enclosing functions' variables this one uses, by where they are
    /// bound, with the variable each is loaded into.
    captures: Vec<((usize, Var), Var)>,
}

impl Builder {
    /// A function whose parameters are named `params`; the first is the
    /// environment, except in a chunk's entry.
    fn new(id: FnId, name: &str, params: Vec<&str>) -> Self {
        let mut builder = Self {
            id,
            name: name.to_string(),
            params: Vec::new(),
            vars: Vec::new(),
            blocks: vec![(Vec::new(), Vec::new(), None)],
            current: 0,
            prologue: Vec::new(),
            captures: Vec::new(),
        };
        for param in params {
            let var = builder.var(param);
            builder.params.push(var);
        }
        builder
    }

    fn var(&mut self, name: &str) -> Var {
        self.vars.push(name.to_string());
        self.vars.len() - 1
    }

    fn block(&mut self, params: Vec<Var>) -> BlockId {
        self.blocks.push((params, Vec::new(), None));
        self.blocks.len() - 1
    }

    fn finish(mut self) -> Function {
        let prologue = std::mem::take(&mut self.prologue);
        self.blocks[0].1.splice(0..0, prologue);
        Function {
            id: self.id,
            name: self.name,
            params: self.params,
            vars: self.vars,
            blocks: self
                .blocks
                .into_iter()
                .map(|(params, stmts, term)| Block {
                    params,
                    stmts,
                    term: term.expect("every block is terminated"),
                })
                .collect(),
        }
    }
}

/// Where the value of an expression goes.
#[derive(Clone, Copy)]
enum Dest {
    /// Returned from the current function.
    Return,
    /// Passed to a join block.
    Jump(BlockId),
}

/// A function about to be compiled: `fun` clauses, or `fn` rules with one
/// parameter each.
struct Definition<'a> {
    name: &'a str,
    clauses: Vec<(Vec<&'a Pat>, &'a Expr)>,
    /// Where a match failure in it is reported.
    location: String,
}

/// An application lowered up to its last call.
enum Applied {
    Value(Atom),
    Call(Callee, Vec<Atom>),
}

/// Built-in functions that are compiled inline when applied.
#[derive(Clone, Copy)]
enum Builtin {
    Print,
    IntToString,
    Size,
    Not,
    Negate,
    Ref,
    Deref,
    Ignore,
}

fn builtin(name: &str) -> Option<Builtin> {
    Some(match name {
        "print" => Builtin::Print,
        "Int.toString" => Builtin::IntToString,
        "size" => Builtin::Size,
        "not" => Builtin::Not,
        "~" => Builtin::Negate,
        "ref" => Builtin::Ref,
        "!" => Builtin::Deref,
        "ignore" => Builtin::Ignore,
        _ => return None,
    })
}

struct Lowerer<'a> {
    session: &'a mut Session,
    types: &'a TypeTable,
    source: &'a Source,
    new_globals: Vec<(GlobalId, String)>,
    /// Finished functions.
    functions: Vec<Function>,
    /// The functions being lowered, innermost last; the chunk's entry first.
    frames: Vec<Builder>,
    /// Names bound inside the functions being lowered, latest last.
    scope: Vec<(String, Binding)>,
}

impl Lowerer<'_> {
    fn frame(&mut self) -> &mut Builder {
        self.frames.last_mut().expect("a function is being lowered")
    }

    fn depth(&self) -> usize {
        self.frames.len() - 1
    }

    fn emit(&mut self, stmt: Stmt) {
        let frame = self.frame();
        let current = frame.current;
        frame.blocks[current].1.push(stmt);
    }

    /// Names the result of `op`.
    fn bind(&mut self, name: &str, op: Op) -> Atom {
        let var = self.frame().var(name);
        self.emit(Stmt::Let(var, op));
        Atom::Var(var)
    }

    /// A variable holding `atom`, for binding a name to it.
    fn var_for(&mut self, name: &str, atom: Atom) -> Var {
        match atom {
            Atom::Var(var) => var,
            atom => match self.bind(name, Op::Atom(atom)) {
                Atom::Var(var) => var,
                _ => unreachable!(),
            },
        }
    }

    fn terminate(&mut self, term: Term) {
        let frame = self.frame();
        let current = frame.current;
        debug_assert!(frame.blocks[current].2.is_none(), "block terminated twice");
        frame.blocks[current].2 = Some(term);
    }

    fn switch(&mut self, block: BlockId) {
        self.frame().current = block;
    }

    fn block(&mut self, params: Vec<Var>) -> BlockId {
        self.frame().block(params)
    }

    /// Continues in a new block when `condition` holds, else jumps to `fail`.
    fn test(&mut self, condition: Atom, fail: BlockId) {
        let next = self.block(Vec::new());
        self.terminate(Term::If(condition, next, fail));
        self.switch(next);
    }

    fn lookup(&self, name: &str) -> Option<Binding> {
        self.scope
            .iter()
            .rev()
            .find(|(known, _)| known == name)
            .map(|(_, binding)| binding.clone())
            .or_else(|| self.session.lookup(name))
    }

    /// The variable of the current function that holds variable `var` of the
    /// function at `depth`, capturing it into every function in between.
    fn access(&mut self, depth: usize, var: Var) -> Var {
        self.capture(self.depth(), depth, var)
    }

    fn capture(&mut self, at: usize, depth: usize, var: Var) -> Var {
        if at == depth {
            return var;
        }
        let frame = &mut self.frames[at];
        if let Some((_, found)) = frame.captures.iter().find(|(key, _)| *key == (depth, var)) {
            return *found;
        }
        // Field 0 of a closure is its code.
        let index = frame.captures.len() + 1;
        let name = self.frames[depth].vars[var].clone();
        let frame = &mut self.frames[at];
        let loaded = frame.var(&name);
        let env = frame.params[0];
        frame
            .prologue
            .push(Stmt::Let(loaded, Op::Select(Atom::Var(env), index)));
        frame.captures.push(((depth, var), loaded));
        loaded
    }

    /// The value of a bound name.
    fn load(&mut self, name: &str, binding: &Binding) -> Atom {
        match binding {
            Binding::Global(global, _) => self.bind(name, Op::Global(*global)),
            Binding::Local { depth, var, .. } => Atom::Var(self.access(*depth, *var)),
        }
    }

    fn ty(&self, expr: &Expr) -> Option<&Ty> {
        self.types.expr(expr)
    }

    fn is_real(&self, expr: &Expr) -> bool {
        self.ty(expr).is_some_and(|ty| ty.is("real"))
    }

    // --- declarations ----------------------------------------------------

    fn statement(&mut self, statement: &StmtKind) -> Res<()> {
        match statement {
            StmtKind::Val(name, expr) => {
                let value = self.value(expr)?;
                self.bind_top(name, value, None);
            }
            StmtKind::Print(expr) => {
                let text = self.value(expr)?;
                self.bind("", Op::Prim(Prim::Print, vec![text]));
            }
            StmtKind::Exit(expr) => {
                self.value(expr)?;
            }
            StmtKind::Declaration(declaration) => self.declaration(declaration, true)?,
        }
        Ok(())
    }

    /// Binds the top-level `name` to `value` in a new global.
    fn bind_top(&mut self, name: &str, value: Atom, known: Option<Known>) {
        let global = self.new_global(name, known);
        self.emit(Stmt::SetGlobal(global, value));
    }

    fn new_global(&mut self, name: &str, known: Option<Known>) -> GlobalId {
        let global = self.session.next_global;
        self.session.next_global += 1;
        self.new_globals.push((global, name.to_string()));
        self.session
            .globals
            .push((name.to_string(), Binding::Global(global, known)));
        global
    }

    /// Lowers a declaration. At the top level, the names it binds become
    /// globals; elsewhere they are local variables.
    fn declaration(&mut self, declaration: &Decl, top: bool) -> Res<()> {
        match &declaration.value {
            DeclKind::Val {
                recursive: false,
                bindings,
            } => {
                // Every right-hand side is evaluated before any name is bound.
                let mut values = Vec::new();
                for (_, expr) in bindings {
                    values.push(self.value(expr)?);
                }
                let mut bound = Vec::new();
                for ((pattern, expr), value) in bindings.iter().zip(values) {
                    let fail = self.block(Vec::new());
                    let mark = self.scope.len();
                    self.pattern(pattern, value, fail)?;
                    bound.extend(self.scope.drain(mark..));
                    let next = self.block(Vec::new());
                    self.terminate(Term::Jump(next, Vec::new()));
                    self.switch(fail);
                    let location = format!(
                        "{}:{}-{}",
                        self.source.file,
                        self.source.position(pattern.start.offset),
                        self.source.position(expr.end.offset)
                    );
                    self.terminate(Term::Fail(crate::core::Failure::Bind, location));
                    self.switch(next);
                }
                self.bind_names(bound, top);
            }
            DeclKind::Val {
                recursive: true,
                bindings,
            } => {
                let mut definitions = Vec::new();
                for (pattern, expr) in bindings {
                    let (name, rules) = match (
                        &strip_typed_pattern(pattern).value,
                        &strip_typed(expr).value,
                    ) {
                        (PatKind::Variable(name), ExprKind::Fn(rules)) => (name, rules),
                        _ => return Err(unsupported("val rec bindings that are not fn", expr)),
                    };
                    definitions.push(Definition {
                        name,
                        clauses: rules.iter().map(|(p, body)| (vec![p], body)).collect(),
                        location: self.source.end(expr),
                    });
                }
                self.functions_group(definitions, top)?;
            }
            DeclKind::Fun(bindings) => {
                let definitions = bindings
                    .iter()
                    .map(|binding| Definition {
                        name: &binding.name,
                        clauses: binding
                            .clauses
                            .iter()
                            .map(|clause| (clause.parameters.iter().collect(), &clause.body))
                            .collect(),
                        location: binding
                            .clauses
                            .last()
                            .map_or_else(String::new, |clause| self.source.end(&clause.body)),
                    })
                    .collect();
                self.functions_group(definitions, top)?;
            }
            DeclKind::Local(private, public) => {
                // The private declarations' names are dropped once the
                // public ones are lowered; their values live on.
                let (outer, inner) = if top {
                    let outer = self.session.globals.len();
                    self.declarations(private, top)?;
                    (outer, self.session.globals.len())
                } else {
                    let outer = self.scope.len();
                    self.declarations(private, top)?;
                    (outer, self.scope.len())
                };
                self.declarations(public, top)?;
                if top {
                    self.session.globals.drain(outer..inner);
                } else {
                    self.scope.drain(outer..inner);
                }
            }
            // Types have no run-time presence; fixity only affects parsing.
            DeclKind::Type(_) | DeclKind::Fixity { .. } => {}
            other => {
                let what = match other {
                    DeclKind::Datatype { .. } | DeclKind::DatatypeCopy { .. } => {
                        "datatype declarations"
                    }
                    DeclKind::Abstype { .. } => "abstype declarations",
                    DeclKind::Exception(_) => "exception declarations",
                    _ => "module declarations",
                };
                return Err((
                    LowerError::Unsupported(what.into()),
                    declaration.source_span(),
                ));
            }
        }
        Ok(())
    }

    fn declarations(&mut self, declarations: &[Decl], top: bool) -> Res<()> {
        for declaration in declarations {
            self.declaration(declaration, top)?;
        }
        Ok(())
    }

    /// Makes names bound by patterns visible: as globals at the top level.
    fn bind_names(&mut self, bound: Vec<(String, Binding)>, top: bool) {
        for (name, binding) in bound {
            if top {
                let value = self.load(&name, &binding);
                self.bind_top(&name, value, None);
            } else {
                self.scope.push((name, binding));
            }
        }
    }

    /// Compiles an `and` group of mutually recursive functions.
    fn functions_group(&mut self, definitions: Vec<Definition>, top: bool) -> Res<()> {
        let depth = self.depth();
        let mut group = Vec::new();
        for definition in &definitions {
            let arity = definition.clauses[0].0.len();
            let known = Known {
                worker: self.session.function_id(),
                arity,
            };
            // The closure is a variable of the enclosing function; at the top
            // level it is also stored in a global.
            let closure = self.frame().var(definition.name);
            let global = if top {
                Some(self.new_global(definition.name, Some(known)))
            } else {
                self.scope.push((
                    definition.name.to_string(),
                    Binding::Local {
                        depth,
                        var: closure,
                        known: Some(known),
                    },
                ));
                None
            };
            group.push((closure, known, global));
        }
        let mut closures = Vec::new();
        for (definition, (closure, known, _)) in definitions.iter().zip(&group) {
            let captures = self.function(known.worker, definition)?;
            let captured = captures
                .into_iter()
                .map(|(depth, var)| Atom::Var(self.access(depth, var)))
                .collect();
            let code = if known.arity == 1 {
                known.worker
            } else {
                self.curried_entry(definition.name, *known)
            };
            closures.push(Closure {
                var: *closure,
                code,
                captured,
            });
        }
        self.emit(Stmt::Closures(closures));
        for (closure, _, global) in group {
            if let Some(global) = global {
                self.emit(Stmt::SetGlobal(global, Atom::Var(closure)));
            }
        }
        Ok(())
    }

    /// The code of a curried function's closure: stage `k` takes the `k`th
    /// argument and returns a closure holding the arguments so far, and the
    /// last stage calls the worker with all of them. Stage 1's environment
    /// is the function's own closure, which is also the worker's.
    fn curried_entry(&mut self, name: &str, known: Known) -> FnId {
        let stages: Vec<FnId> = (0..known.arity)
            .map(|_| self.session.function_id())
            .collect();
        for (index, id) in stages.iter().enumerate() {
            self.frames.push(Builder::new(*id, name, vec!["env", ""]));
            let env = Atom::Var(self.frame().params[0]);
            let argument = Atom::Var(self.frame().params[1]);
            // The worker's environment and the earlier arguments.
            let mut held = if index == 0 {
                vec![env]
            } else {
                (1..=index + 1)
                    .map(|field| self.bind("", Op::Select(env.clone(), field)))
                    .collect()
            };
            held.push(argument);
            if index + 1 == known.arity {
                let env = held.remove(0);
                self.terminate(Term::TailCall(Callee::Known(known.worker, env), held));
            } else {
                let result = self.closure("", stages[index + 1], held);
                self.terminate(Term::Return(result));
            }
            let frame = self.frames.pop().expect("the stage's frame");
            self.functions.push(frame.finish());
        }
        stages[0]
    }

    /// Compiles `definition` as the function `id`, taking its closure and one
    /// argument per curried parameter; returns the variables it captures.
    fn function(&mut self, id: FnId, definition: &Definition) -> Res<Vec<(usize, Var)>> {
        let arity = definition.clauses[0].0.len();
        let mut params = vec!["env"];
        params.extend(std::iter::repeat_n("", arity));
        self.frames.push(Builder::new(id, definition.name, params));
        let mark = self.scope.len();
        let args: Vec<Atom> = self.frame().params[1..]
            .iter()
            .map(|var| Atom::Var(*var))
            .collect();
        let result = self.rules(
            &args,
            &definition.clauses,
            Dest::Return,
            &definition.location,
        );
        self.scope.truncate(mark);
        let frame = self.frames.pop().expect("the function's frame");
        result?;
        let captures = frame.captures.iter().map(|(key, _)| *key).collect();
        self.functions.push(frame.finish());
        Ok(captures)
    }

    // --- matching --------------------------------------------------------

    /// Matches `scrutinees` against each rule's patterns in turn and sends
    /// the first matching rule's body to `dest`; no match raises `Match`.
    fn rules(
        &mut self,
        scrutinees: &[Atom],
        rules: &[(Vec<&Pat>, &Expr)],
        dest: Dest,
        location: &str,
    ) -> Res<()> {
        for (patterns, body) in rules {
            let fail = self.block(Vec::new());
            let mark = self.scope.len();
            for (pattern, scrutinee) in patterns.iter().zip(scrutinees) {
                self.pattern(pattern, scrutinee.clone(), fail)?;
            }
            self.into(body, dest)?;
            self.scope.truncate(mark);
            self.switch(fail);
        }
        self.terminate(Term::Fail(
            crate::core::Failure::Match,
            location.to_string(),
        ));
        Ok(())
    }

    /// Binds `name` to `value` in the scope.
    fn bind_local(&mut self, name: &str, value: Atom) -> Var {
        let var = self.var_for(name, value);
        // Name an anonymous variable, such as a parameter, for dumps.
        let frame = self.frame();
        if frame.vars[var].is_empty() {
            frame.vars[var] = name.to_string();
        }
        let depth = self.depth();
        self.scope.push((
            name.to_string(),
            Binding::Local {
                depth,
                var,
                known: None,
            },
        ));
        var
    }

    /// Tests `value` against `pattern`, jumping to `fail` if it does not
    /// match, and binds the pattern's variables in the scope.
    fn pattern(&mut self, pattern: &Pat, value: Atom, fail: BlockId) -> Res<()> {
        match &pattern.value {
            PatKind::Wildcard | PatKind::Unit => {}
            PatKind::Variable(name) => match constant(name) {
                Some(word) => {
                    let equal =
                        self.bind("", Op::Prim(Prim::WordEq, vec![value, Atom::Word(word)]));
                    self.test(equal, fail);
                }
                None => {
                    self.bind_local(name, value);
                }
            },
            PatKind::Integer(_) | PatKind::Boolean(_) | PatKind::Character(_) => {
                let word = match &pattern.value {
                    PatKind::Integer(integer) => value::tagged(*integer),
                    PatKind::Boolean(boolean) => value::tagged(i64::from(*boolean)),
                    PatKind::Character(character) => {
                        value::tagged(i64::from(u32::from(*character)))
                    }
                    _ => unreachable!(),
                };
                let equal = self.bind("", Op::Prim(Prim::WordEq, vec![value, Atom::Word(word)]));
                self.test(equal, fail);
            }
            PatKind::String(text) => {
                let equal = self.bind(
                    "",
                    Op::Prim(Prim::Equal, vec![value, Atom::String(text.clone())]),
                );
                self.test(equal, fail);
            }
            PatKind::Tuple(items) => {
                for (index, item) in items.iter().enumerate() {
                    let field = self.bind("", Op::Select(value.clone(), index));
                    self.pattern(item, field, fail)?;
                }
            }
            PatKind::Record(fields, _) => {
                let Some(Ty::Record(labels)) = self.types.pat(pattern).cloned() else {
                    return Err(unsupported_pattern("this record pattern", pattern));
                };
                for (label, item) in fields {
                    let index = labels
                        .iter()
                        .position(|(known, _)| known == label)
                        .expect("the record type has the pattern's labels");
                    let field = self.bind("", Op::Select(value.clone(), index));
                    self.pattern(item, field, fail)?;
                }
            }
            PatKind::List(items) => {
                let mut list = value;
                for item in items {
                    let boxed = self.bind("", Op::Prim(Prim::IsBoxed, vec![list.clone()]));
                    self.test(boxed, fail);
                    let head = self.bind("", Op::Select(list.clone(), 0));
                    self.pattern(item, head, fail)?;
                    list = self.bind("", Op::Select(list, 1));
                }
                let empty = self.bind(
                    "",
                    Op::Prim(Prim::WordEq, vec![list, Atom::Word(value::NIL)]),
                );
                self.test(empty, fail);
            }
            PatKind::Cons(head, tail) => {
                let boxed = self.bind("", Op::Prim(Prim::IsBoxed, vec![value.clone()]));
                self.test(boxed, fail);
                let first = self.bind("", Op::Select(value.clone(), 0));
                self.pattern(head, first, fail)?;
                let rest = self.bind("", Op::Select(value, 1));
                self.pattern(tail, rest, fail)?;
            }
            PatKind::Constructor(name, argument) if name == "::" => {
                // A cons cell is laid out like the pair `(head, tail)`.
                let boxed = self.bind("", Op::Prim(Prim::IsBoxed, vec![value.clone()]));
                self.test(boxed, fail);
                self.pattern(argument, value, fail)?;
            }
            PatKind::Constructor(name, argument) if name == "ref" => {
                let contents = self.bind("", Op::Select(value, 0));
                self.pattern(argument, contents, fail)?;
            }
            PatKind::Layered(name, _, inner) => {
                let var = self.bind_local(name, value);
                self.pattern(inner, Atom::Var(var), fail)?;
            }
            PatKind::Typed(inner, _) => self.pattern(inner, value, fail)?,
            PatKind::Word(_) => return Err(unsupported_pattern("word patterns", pattern)),
            PatKind::Constructor(..) => {
                return Err(unsupported_pattern(
                    "datatype constructor patterns",
                    pattern,
                ));
            }
        }
        Ok(())
    }

    // --- expressions -----------------------------------------------------

    /// Lowers `expr` and sends its value to `dest`, ending the current block.
    fn into(&mut self, expr: &Expr, dest: Dest) -> Res<()> {
        match &expr.value {
            ExprKind::If(condition, consequent, alternative) => {
                let condition = self.value(condition)?;
                let then = self.block(Vec::new());
                let otherwise = self.block(Vec::new());
                self.terminate(Term::If(condition, then, otherwise));
                self.switch(then);
                self.into(consequent, dest)?;
                self.switch(otherwise);
                self.into(alternative, dest)
            }
            ExprKind::Case(scrutinee, rules) => {
                let scrutinee = self.value(scrutinee)?;
                let rules: Vec<(Vec<&Pat>, &Expr)> = rules
                    .iter()
                    .map(|(pattern, body)| (vec![pattern], body))
                    .collect();
                let location = self.source.end(expr);
                self.rules(&[scrutinee], &rules, dest, &location)
            }
            ExprKind::Typed(inner, _) => self.into(inner, dest),
            ExprKind::Let(declarations, body) => {
                let mark = self.scope.len();
                self.declarations(declarations, false)?;
                self.into(body, dest)?;
                self.scope.truncate(mark);
                Ok(())
            }
            // A call whose value is returned is a tail call: it reuses the
            // caller's frame, so loops written as recursion run in constant
            // stack.
            ExprKind::Apply(..) if matches!(dest, Dest::Return) => {
                match self.application(expr)? {
                    Applied::Value(value) => self.send(value, dest),
                    Applied::Call(callee, args) => self.terminate(Term::TailCall(callee, args)),
                }
                Ok(())
            }
            ExprKind::Sequence(items) => {
                let (last, rest) = items.split_last().expect("a sequence has an expression");
                for item in rest {
                    self.value(item)?;
                }
                self.into(last, dest)
            }
            _ => {
                let value = self.value(expr)?;
                self.send(value, dest);
                Ok(())
            }
        }
    }

    fn send(&mut self, value: Atom, dest: Dest) {
        match dest {
            Dest::Return => self.terminate(Term::Return(value)),
            Dest::Jump(join) => self.terminate(Term::Jump(join, vec![value])),
        }
    }

    /// Lowers a branching expression through a join block.
    fn joined(&mut self, expr: &Expr) -> Res<Atom> {
        let result = self.frame().var("");
        let join = self.block(vec![result]);
        self.into(expr, Dest::Jump(join))?;
        self.switch(join);
        Ok(Atom::Var(result))
    }

    /// Lowers `expr` in the current block and returns its value.
    fn value(&mut self, expr: &Expr) -> Res<Atom> {
        Ok(match &expr.value {
            ExprKind::Integer(value) => Atom::Word(value::tagged(*value)),
            ExprKind::Real(value) => Atom::Real(*value),
            ExprKind::Boolean(value) => Atom::Word(value::tagged(i64::from(*value))),
            ExprKind::Character(value) => Atom::Word(value::tagged(i64::from(u32::from(*value)))),
            ExprKind::String(value) => Atom::String(value.clone()),
            ExprKind::Unit => Atom::Word(value::tagged(0)),
            ExprKind::Variable(name) => match self.lookup(name) {
                Some(binding) => self.load(name, &binding),
                None => match (constant(name), builtin(name)) {
                    (Some(word), _) => Atom::Word(word),
                    (None, Some(builtin)) => self.builtin_closure(name, builtin),
                    (None, None) => {
                        return Err(unsupported(
                            &format!("built-in functions used as values, such as {name},"),
                            expr,
                        ));
                    }
                },
            },
            ExprKind::Tuple(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value(item))
                    .collect::<Res<Vec<_>>>()?;
                self.bind("", Op::Record(items))
            }
            ExprKind::Record(fields) => {
                // Fields are evaluated in source order and stored in label
                // order.
                let Some(Ty::Record(labels)) = self.ty(expr).cloned() else {
                    return Err(unsupported("this record", expr));
                };
                let mut values = Vec::new();
                for (label, field) in fields {
                    values.push((label.clone(), self.value(field)?));
                }
                let ordered = labels
                    .iter()
                    .map(|(label, _)| {
                        values
                            .iter()
                            .find(|(known, _)| known == label)
                            .map(|(_, value)| value.clone())
                            .expect("the record has every label of its type")
                    })
                    .collect();
                self.bind("", Op::Record(ordered))
            }
            ExprKind::List(items) => {
                let items = items
                    .iter()
                    .map(|item| self.value(item))
                    .collect::<Res<Vec<_>>>()?;
                let mut list = Atom::Word(value::NIL);
                for item in items.into_iter().rev() {
                    list = self.bind("", Op::Record(vec![item, list]));
                }
                list
            }
            ExprKind::If(..) | ExprKind::Case(..) => self.joined(expr)?,
            ExprKind::AndAlso(lhs, rhs) | ExprKind::OrElse(lhs, rhs) => {
                let result = self.frame().var("");
                let join = self.block(vec![result]);
                let condition = self.value(lhs)?;
                let evaluate = self.block(Vec::new());
                let skip = self.block(Vec::new());
                let (then, otherwise, short) = match &expr.value {
                    ExprKind::AndAlso(..) => (evaluate, skip, value::FALSE),
                    _ => (skip, evaluate, value::TRUE),
                };
                self.terminate(Term::If(condition, then, otherwise));
                self.switch(skip);
                self.terminate(Term::Jump(join, vec![Atom::Word(short)]));
                self.switch(evaluate);
                self.into(rhs, Dest::Jump(join))?;
                self.switch(join);
                Atom::Var(result)
            }
            ExprKind::Sequence(items) => {
                let mut last = Atom::Word(value::tagged(0));
                for item in items {
                    last = self.value(item)?;
                }
                last
            }
            ExprKind::Typed(inner, _) => self.value(inner)?,
            ExprKind::Let(declarations, body) => {
                let mark = self.scope.len();
                self.declarations(declarations, false)?;
                let value = self.value(body)?;
                self.scope.truncate(mark);
                value
            }
            ExprKind::Add(lhs, rhs)
            | ExprKind::Subtract(lhs, rhs)
            | ExprKind::Multiply(lhs, rhs)
            | ExprKind::Divide(lhs, rhs)
            | ExprKind::IntDivide(lhs, rhs)
            | ExprKind::Greater(lhs, rhs)
            | ExprKind::GreaterEqual(lhs, rhs)
            | ExprKind::Less(lhs, rhs)
            | ExprKind::LessEqual(lhs, rhs) => {
                let real = self.is_real(lhs);
                let prim = match (&expr.value, real) {
                    (ExprKind::Add(..), false) => Prim::IntAdd,
                    (ExprKind::Subtract(..), false) => Prim::IntSub,
                    (ExprKind::Multiply(..), false) => Prim::IntMul,
                    (ExprKind::IntDivide(..), _) => Prim::IntDiv,
                    (ExprKind::Greater(..), false) => Prim::IntGt,
                    (ExprKind::GreaterEqual(..), false) => Prim::IntGe,
                    (ExprKind::Less(..), false) => Prim::IntLt,
                    (ExprKind::LessEqual(..), false) => Prim::IntLe,
                    (ExprKind::Add(..), true) => Prim::RealAdd,
                    (ExprKind::Subtract(..), true) => Prim::RealSub,
                    (ExprKind::Multiply(..), true) => Prim::RealMul,
                    (ExprKind::Divide(..), _) => Prim::RealDiv,
                    (ExprKind::Greater(..), true) => Prim::RealGt,
                    (ExprKind::GreaterEqual(..), true) => Prim::RealGe,
                    (ExprKind::Less(..), true) => Prim::RealLt,
                    (ExprKind::LessEqual(..), true) => Prim::RealLe,
                    _ => unreachable!(),
                };
                let mut args = vec![self.value(lhs)?, self.value(rhs)?];
                if prim == Prim::IntDiv {
                    args.push(Atom::String(self.source.operator("div", lhs, rhs)));
                }
                self.bind("", Op::Prim(prim, args))
            }
            ExprKind::Equal(lhs, rhs) | ExprKind::NotEqual(lhs, rhs) => {
                // Immediates are equal when their words are; anything else
                // is compared by the runtime.
                let immediate = self
                    .ty(lhs)
                    .is_some_and(|ty| ty.is("int") || ty.is("bool") || ty.is("char"));
                let prim = match (immediate, matches!(expr.value, ExprKind::Equal(..))) {
                    (true, true) => Prim::WordEq,
                    (true, false) => Prim::WordNe,
                    (false, true) => Prim::Equal,
                    (false, false) => Prim::Unequal,
                };
                let lhs = self.value(lhs)?;
                let rhs = self.value(rhs)?;
                self.bind("", Op::Prim(prim, vec![lhs, rhs]))
            }
            ExprKind::Infix(name, lhs, rhs) => self.infix(name, lhs, rhs, expr)?,
            ExprKind::Fn(rules) => {
                let definition = Definition {
                    name: "fn",
                    clauses: rules.iter().map(|(p, body)| (vec![p], body)).collect(),
                    location: self.source.end(expr),
                };
                self.lambda("", &definition)?
            }
            ExprKind::While(condition, body) => {
                let header = self.block(Vec::new());
                let looped = self.block(Vec::new());
                let done = self.block(Vec::new());
                self.terminate(Term::Jump(header, Vec::new()));
                self.switch(header);
                let condition = self.value(condition)?;
                self.terminate(Term::If(condition, looped, done));
                self.switch(looped);
                self.value(body)?;
                self.terminate(Term::Jump(header, Vec::new()));
                self.switch(done);
                Atom::Word(value::tagged(0))
            }
            ExprKind::Apply(..) => self.apply(expr)?,
            ExprKind::PosixExit(word8) => {
                let ExprKind::Word8FromInt(status) = &word8.value else {
                    return Err(unsupported(
                        "Posix.Process.exit without Word8.fromInt",
                        expr,
                    ));
                };
                let status = self.value(status)?;
                self.bind("", Op::Prim(Prim::Exit, vec![status]))
            }
            other => return Err(unsupported(unsupported_name(other), expr)),
        })
    }

    fn infix(&mut self, name: &str, lhs: &Expr, rhs: &Expr, expr: &Expr) -> Res<Atom> {
        let prim = match name {
            "::" => None,
            "^" => Some(Prim::Concat),
            "mod" => Some(Prim::IntMod),
            ":=" => Some(Prim::Assign),
            "o" | "before" => None,
            _ => {
                // A function declared infix is called with the pair.
                let Some(binding) = self.lookup(name) else {
                    return Err(unsupported(&format!("the infix operator {name}"), expr));
                };
                let pair = vec![self.value(lhs)?, self.value(rhs)?];
                let pair = self.bind("", Op::Record(pair));
                let known = match &binding {
                    Binding::Global(_, known) | Binding::Local { known, .. } => *known,
                };
                let function = self.load(name, &binding);
                let callee = match known {
                    Some(Known { worker, arity: 1 }) => Callee::Known(worker, function),
                    _ => Callee::Closure(function),
                };
                return Ok(self.bind("", Op::Call(callee, vec![pair])));
            }
        };
        let mut args = vec![self.value(lhs)?, self.value(rhs)?];
        if name == "o" {
            let g = args.pop().expect("two operands");
            let f = args.pop().expect("two operands");
            return Ok(self.compose(f, g));
        }
        if name == "before" {
            return Ok(args.swap_remove(0));
        }
        Ok(match prim {
            // A cons cell is a record of the head and the tail.
            None => self.bind("", Op::Record(args)),
            Some(prim) => {
                if prim == Prim::IntMod {
                    args.push(Atom::String(self.source.operator("mod", lhs, rhs)));
                }
                self.bind("", Op::Prim(prim, args))
            }
        })
    }

    /// Lowers an application.
    fn apply(&mut self, expr: &Expr) -> Res<Atom> {
        Ok(match self.application(expr)? {
            Applied::Value(value) => value,
            Applied::Call(callee, args) => self.bind("", Op::Call(callee, args)),
        })
    }

    /// Lowers an application up to its last call, which is left to the
    /// caller so that a call in tail position can become a tail call.
    fn application(&mut self, expr: &Expr) -> Res<Applied> {
        let mut args = Vec::new();
        let mut head = expr;
        while let ExprKind::Apply(function, argument) = &head.value {
            args.push(argument.as_ref());
            head = function;
        }
        args.reverse();
        let head = strip_typed(head);
        let mut rest = args.as_slice();
        let mut function = None;
        if let ExprKind::Variable(name) = &head.value {
            let binding = self.lookup(name);
            if binding.is_none()
                && let Some(builtin) = builtin(name)
            {
                let real = self.is_real(args[0]);
                let argument = self.value(args[0])?;
                function = Some(match builtin {
                    Builtin::Negate if real => {
                        self.bind("", Op::Prim(Prim::RealNeg, vec![argument]))
                    }
                    builtin => self.builtin(builtin, argument),
                });
                rest = &args[1..];
            } else if let Some(binding) = binding {
                let known = match &binding {
                    Binding::Global(_, known) | Binding::Local { known, .. } => *known,
                };
                if let Some(known) = known
                    && args.len() >= known.arity
                {
                    let closure = self.load(name, &binding);
                    let values = args[..known.arity]
                        .iter()
                        .map(|arg| self.value(arg))
                        .collect::<Res<Vec<_>>>()?;
                    let mut call = Applied::Call(Callee::Known(known.worker, closure), values);
                    for argument in &args[known.arity..] {
                        let function = self.finish_call(call);
                        let argument = self.value(argument)?;
                        call = Applied::Call(Callee::Closure(function), vec![argument]);
                    }
                    return Ok(call);
                }
            }
        }
        if let ExprKind::Selector(label) = &head.value {
            let record = self.value(args[0])?;
            let Some(Ty::Record(labels)) = self.ty(args[0]).cloned() else {
                return Err(unsupported("selectors on records of unknown shape", expr));
            };
            let index = labels
                .iter()
                .position(|(known, _)| known == label)
                .expect("the record type has the selected label");
            function = Some(self.bind("", Op::Select(record, index)));
            rest = &args[1..];
        }
        // Anything else is a closure called one argument at a time.
        let mut call = Applied::Value(match function {
            Some(function) => function,
            None => self.value(head)?,
        });
        for argument in rest {
            let function = self.finish_call(call);
            let argument = self.value(argument)?;
            call = Applied::Call(Callee::Closure(function), vec![argument]);
        }
        Ok(call)
    }

    /// The value of an application, making its pending call.
    fn finish_call(&mut self, applied: Applied) -> Atom {
        match applied {
            Applied::Value(value) => value,
            Applied::Call(callee, args) => self.bind("", Op::Call(callee, args)),
        }
    }

    fn builtin(&mut self, builtin: Builtin, argument: Atom) -> Atom {
        match builtin {
            Builtin::Print => self.bind("", Op::Prim(Prim::Print, vec![argument])),
            Builtin::IntToString => self.bind("", Op::Prim(Prim::IntToString, vec![argument])),
            Builtin::Size => self.bind("", Op::Prim(Prim::Size, vec![argument])),
            Builtin::Not => self.bind(
                "",
                Op::Prim(Prim::WordEq, vec![argument, Atom::Word(value::FALSE)]),
            ),
            Builtin::Negate => self.bind("", Op::Prim(Prim::IntNeg, vec![argument])),
            Builtin::Ref => self.bind("", Op::Prim(Prim::Ref, vec![argument])),
            Builtin::Deref => self.bind("", Op::Select(argument, 0)),
            Builtin::Ignore => Atom::Word(value::tagged(0)),
        }
    }

    /// A closure for a built-in function used as a value.
    fn builtin_closure(&mut self, name: &str, builtin: Builtin) -> Atom {
        let id = self.session.function_id();
        self.frames.push(Builder::new(id, name, vec!["env", ""]));
        let argument = Atom::Var(self.frame().params[1]);
        let result = self.builtin(builtin, argument);
        self.terminate(Term::Return(result));
        let frame = self.frames.pop().expect("the builtin's frame");
        self.functions.push(frame.finish());
        self.closure(name, id, Vec::new())
    }

    /// Allocates a closure of the function `code`.
    fn closure(&mut self, name: &str, code: FnId, captured: Vec<Atom>) -> Atom {
        let var = self.frame().var(name);
        self.emit(Stmt::Closures(vec![Closure {
            var,
            code,
            captured,
        }]));
        Atom::Var(var)
    }

    /// Compiles an anonymous function and allocates its closure.
    fn lambda(&mut self, name: &str, definition: &Definition) -> Res<Atom> {
        let id = self.session.function_id();
        let captures = self.function(id, definition)?;
        let captured = captures
            .into_iter()
            .map(|(depth, var)| Atom::Var(self.access(depth, var)))
            .collect();
        Ok(self.closure(name, id, captured))
    }

    /// `f o g`: a closure calling `g`, then `f`, both captured.
    fn compose(&mut self, f: Atom, g: Atom) -> Atom {
        let id = self.session.function_id();
        self.frames.push(Builder::new(id, "o", vec!["env", ""]));
        let env = Atom::Var(self.frame().params[0]);
        let argument = Atom::Var(self.frame().params[1]);
        let f_inner = self.bind("f", Op::Select(env.clone(), 1));
        let g_inner = self.bind("g", Op::Select(env, 2));
        let inner = self.bind("", Op::Call(Callee::Closure(g_inner), vec![argument]));
        self.terminate(Term::TailCall(Callee::Closure(f_inner), vec![inner]));
        let frame = self.frames.pop().expect("the composition's frame");
        self.functions.push(frame.finish());
        self.closure("", id, vec![f, g])
    }
}

/// The word of a built-in nullary constructor.
fn constant(name: &str) -> Option<i64> {
    match name {
        "nil" => Some(value::NIL),
        "true" => Some(value::TRUE),
        "false" => Some(value::FALSE),
        _ => None,
    }
}

fn strip_typed(expr: &Expr) -> &Expr {
    match &expr.value {
        ExprKind::Typed(inner, _) => strip_typed(inner),
        _ => expr,
    }
}

fn strip_typed_pattern(pattern: &Pat) -> &Pat {
    match &pattern.value {
        PatKind::Typed(inner, _) => strip_typed_pattern(inner),
        _ => pattern,
    }
}

fn unsupported(what: &str, expr: &Expr) -> Failure {
    (
        LowerError::Unsupported(what.to_string()),
        expr.source_span(),
    )
}

fn unsupported_pattern(what: &str, pattern: &Pat) -> Failure {
    (
        LowerError::Unsupported(what.to_string()),
        pattern.source_span(),
    )
}

fn unsupported_name(kind: &ExprKind) -> &'static str {
    match kind {
        ExprKind::Word(_) => "word literals",
        ExprKind::Selector(_) => "record selectors used as values",
        ExprKind::Raise(_) => "raise",
        ExprKind::Handle(..) => "handle",
        _ => "this expression",
    }
}
