//! Exhaustiveness and redundancy checking for `case`, `fn` and `handle`.
//!
//! This is the classic pattern-matrix "usefulness" algorithm. It works on the
//! shape of the patterns alone, before types are known, so it only knows the
//! constructor families of the built-in types (`bool`, `list`, `option`,
//! `order`, and single-constructor tuples and records). Any other constructor
//! belongs to an open family: it can be matched redundantly, but never
//! exhaustively without a catch-all. Datatype declarations will extend
//! `family` once they exist.
//!
//! As in SML/NJ, a redundant rule is an error and a non-exhaustive match is
//! only a warning (`handle` re-raises, so it is never non-exhaustive).

use crate::parser::{Expr, ExprKind, Pat, PatKind, Program, Rule, StmtKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchDiagnosticKind {
    Redundant,
    NonExhaustive,
}

pub struct MatchDiagnostic {
    pub kind: MatchDiagnosticKind,
    pub line: usize,
    pub column: usize,
    pub span: miette::SourceSpan,
}

pub fn check_program(program: &Program) -> Vec<MatchDiagnostic> {
    let mut diagnostics = Vec::new();
    for statement in &program.statements {
        match &statement.value {
            StmtKind::Val(_, expr) | StmtKind::Print(expr) | StmtKind::Exit(expr) => {
                visit(expr, &mut diagnostics);
            }
        }
    }
    diagnostics
}

fn visit(expr: &Expr, out: &mut Vec<MatchDiagnostic>) {
    match &expr.value {
        ExprKind::Add(lhs, rhs)
        | ExprKind::Subtract(lhs, rhs)
        | ExprKind::Multiply(lhs, rhs)
        | ExprKind::Divide(lhs, rhs)
        | ExprKind::IntDivide(lhs, rhs)
        | ExprKind::Greater(lhs, rhs)
        | ExprKind::GreaterEqual(lhs, rhs)
        | ExprKind::Less(lhs, rhs)
        | ExprKind::LessEqual(lhs, rhs)
        | ExprKind::Equal(lhs, rhs)
        | ExprKind::NotEqual(lhs, rhs)
        | ExprKind::Infix(_, lhs, rhs)
        | ExprKind::AndAlso(lhs, rhs)
        | ExprKind::OrElse(lhs, rhs)
        | ExprKind::Apply(lhs, rhs)
        | ExprKind::While(lhs, rhs) => {
            visit(lhs, out);
            visit(rhs, out);
        }
        ExprKind::If(condition, consequent, alternative) => {
            visit(condition, out);
            visit(consequent, out);
            visit(alternative, out);
        }
        ExprKind::List(items) | ExprKind::Tuple(items) | ExprKind::Sequence(items) => {
            items.iter().for_each(|item| visit(item, out));
        }
        ExprKind::Record(fields) => fields.iter().for_each(|(_, value)| visit(value, out)),
        ExprKind::Word8FromInt(inner)
        | ExprKind::PosixExit(inner)
        | ExprKind::Raise(inner)
        | ExprKind::Typed(inner, _) => visit(inner, out),
        ExprKind::Let(bindings, body) => {
            bindings.iter().for_each(|(_, value)| visit(value, out));
            visit(body, out);
        }
        ExprKind::Case(scrutinee, rules) => {
            visit(scrutinee, out);
            check_rules(rules, expr, true, out);
            rules.iter().for_each(|(_, body)| visit(body, out));
        }
        ExprKind::Fn(rules) => {
            check_rules(rules, expr, true, out);
            rules.iter().for_each(|(_, body)| visit(body, out));
        }
        ExprKind::Handle(body, rules) => {
            visit(body, out);
            check_rules(rules, expr, false, out);
            rules
                .iter()
                .for_each(|(_, rule_body)| visit(rule_body, out));
        }
        ExprKind::Integer(_)
        | ExprKind::Real(_)
        | ExprKind::Boolean(_)
        | ExprKind::Variable(_)
        | ExprKind::String(_)
        | ExprKind::Character(_)
        | ExprKind::Word(_)
        | ExprKind::Unit
        | ExprKind::Selector(_) => {}
    }
}

fn check_rules(
    rules: &[Rule],
    whole: &Expr,
    must_be_exhaustive: bool,
    out: &mut Vec<MatchDiagnostic>,
) {
    let mut labels = Vec::new();
    for (pattern, _) in rules {
        collect_labels(pattern, &mut labels);
    }
    labels.sort_by(|a, b| label_order(a, b));
    labels.dedup();
    let rows: Vec<Vec<P>> = rules
        .iter()
        .map(|(pattern, _)| vec![convert(pattern, &labels)])
        .collect();

    for (index, (pattern, _)) in rules.iter().enumerate() {
        if !useful(&rows[..index], &rows[index]) {
            out.push(MatchDiagnostic {
                kind: MatchDiagnosticKind::Redundant,
                line: pattern.start.line,
                column: pattern.start.column,
                span: pattern.source_span(),
            });
        }
    }
    if must_be_exhaustive && useful(&rows, &[P::Wild]) {
        out.push(MatchDiagnostic {
            kind: MatchDiagnosticKind::NonExhaustive,
            line: whole.start.line,
            column: whole.start.column,
            span: whole.source_span(),
        });
    }
}

/// Numeric labels (tuple fields) sort numerically, the rest alphabetically.
fn label_order(a: &str, b: &str) -> std::cmp::Ordering {
    match (a.parse::<u64>(), b.parse::<u64>()) {
        (Ok(a), Ok(b)) => a.cmp(&b),
        (Ok(_), Err(_)) => std::cmp::Ordering::Less,
        (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
        (Err(_), Err(_)) => a.cmp(b),
    }
}

fn collect_labels(pattern: &Pat, labels: &mut Vec<String>) {
    match &pattern.value {
        PatKind::Record(fields, _) => {
            for (label, inner) in fields {
                labels.push(label.clone());
                collect_labels(inner, labels);
            }
        }
        PatKind::Tuple(items) | PatKind::List(items) => {
            items.iter().for_each(|item| collect_labels(item, labels));
        }
        PatKind::Constructor(_, inner)
        | PatKind::Layered(_, _, inner)
        | PatKind::Typed(inner, _) => collect_labels(inner, labels),
        PatKind::Cons(head, tail) => {
            collect_labels(head, labels);
            collect_labels(tail, labels);
        }
        _ => {}
    }
}

/// A pattern reduced to wildcards and constructors.
#[derive(Clone, Debug)]
enum P {
    Wild,
    Con(Head, Vec<P>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Head {
    /// A named constructor and its argument count (0 or 1, or 2 for `::`).
    Named(String, usize),
    /// Tuples, records and unit: the only constructor of their type.
    Tuple(usize),
    /// A constant of an infinite type: never enumerable.
    Literal(String),
}

/// The constructors of a built-in type, given the name of one of them.
fn family(name: &str) -> Option<&'static [(&'static str, usize)]> {
    match name {
        "true" | "false" => Some(&[("true", 0), ("false", 0)]),
        "nil" | "::" => Some(&[("nil", 0), ("::", 2)]),
        "NONE" | "SOME" => Some(&[("NONE", 0), ("SOME", 1)]),
        "LESS" | "EQUAL" | "GREATER" => Some(&[("LESS", 0), ("EQUAL", 0), ("GREATER", 0)]),
        _ => None,
    }
}

/// Without an environment, a capitalised or qualified name (`Div`, `Foo.Bar`) is
/// taken to be a constructor, such as a Basis exception. Taking it for a variable
/// instead would report the rules after it as redundant. Once datatype and
/// exception declarations exist this should look the name up instead.
fn looks_like_constructor(name: &str) -> bool {
    name.contains('.') || name.starts_with(|first: char| first.is_ascii_uppercase())
}

fn convert(pattern: &Pat, labels: &[String]) -> P {
    let named = |name: &str, arguments: Vec<P>| {
        P::Con(Head::Named(name.into(), arguments.len()), arguments)
    };
    match &pattern.value {
        PatKind::Wildcard => P::Wild,
        PatKind::Variable(name) => match family(name) {
            // A nullary constructor of a built-in type, not a variable.
            Some(constructors) if constructors.contains(&(name.as_str(), 0)) => named(name, vec![]),
            _ if looks_like_constructor(name) => named(name, vec![]),
            _ => P::Wild,
        },
        PatKind::Integer(value) => P::Con(Head::Literal(format!("int {value}")), vec![]),
        PatKind::Word(value) => P::Con(Head::Literal(format!("word {value}")), vec![]),
        PatKind::String(value) => P::Con(Head::Literal(format!("string {value}")), vec![]),
        PatKind::Character(value) => P::Con(Head::Literal(format!("char {value}")), vec![]),
        PatKind::Boolean(value) => named(if *value { "true" } else { "false" }, vec![]),
        PatKind::Unit => P::Con(Head::Tuple(0), vec![]),
        PatKind::Tuple(items) => P::Con(
            Head::Tuple(items.len()),
            items.iter().map(|item| convert(item, labels)).collect(),
        ),
        PatKind::List(items) => items.iter().rev().fold(named("nil", vec![]), |tail, item| {
            named("::", vec![convert(item, labels), tail])
        }),
        PatKind::Record(fields, _) => P::Con(
            Head::Tuple(labels.len()),
            labels
                .iter()
                .map(|label| {
                    fields
                        .iter()
                        .find(|(field, _)| field == label)
                        .map_or(P::Wild, |(_, inner)| convert(inner, labels))
                })
                .collect(),
        ),
        PatKind::Constructor(name, argument) => named(name, vec![convert(argument, labels)]),
        PatKind::Cons(head, tail) => {
            named("::", vec![convert(head, labels), convert(tail, labels)])
        }
        PatKind::Layered(_, _, inner) | PatKind::Typed(inner, _) => convert(inner, labels),
    }
}

fn wildcards(count: usize) -> Vec<P> {
    vec![P::Wild; count]
}

/// The rows that can still match once the first column is known to be `head`.
fn specialize(rows: &[Vec<P>], head: &Head, arity: usize) -> Vec<Vec<P>> {
    rows.iter()
        .filter_map(|row| match &row[0] {
            P::Con(found, arguments) if found == head => {
                Some(arguments.iter().chain(&row[1..]).cloned().collect())
            }
            P::Con(..) => None,
            P::Wild => Some(
                wildcards(arity)
                    .into_iter()
                    .chain(row[1..].iter().cloned())
                    .collect(),
            ),
        })
        .collect()
}

/// The rows whose first column matches anything.
fn default_rows(rows: &[Vec<P>]) -> Vec<Vec<P>> {
    rows.iter()
        .filter(|row| matches!(row[0], P::Wild))
        .map(|row| row[1..].to_vec())
        .collect()
}

/// Whether some value matches `vector` but none of `rows`.
fn useful(rows: &[Vec<P>], vector: &[P]) -> bool {
    let Some((first, rest)) = vector.split_first() else {
        return rows.is_empty();
    };
    match first {
        P::Con(head, arguments) => {
            let arity = arguments.len();
            let specialized = specialize(rows, head, arity);
            let next: Vec<P> = arguments.iter().chain(rest).cloned().collect();
            useful(&specialized, &next)
        }
        P::Wild => {
            let heads: Vec<&Head> = rows
                .iter()
                .filter_map(|row| match &row[0] {
                    P::Con(head, _) => Some(head),
                    P::Wild => None,
                })
                .collect();
            match complete_signature(&heads) {
                Some(constructors) => constructors.iter().any(|(head, arity)| {
                    let specialized = specialize(rows, head, *arity);
                    let next: Vec<P> = wildcards(*arity)
                        .into_iter()
                        .chain(rest.iter().cloned())
                        .collect();
                    useful(&specialized, &next)
                }),
                None => useful(&default_rows(rows), rest),
            }
        }
    }
}

/// All constructors of the column's type, when the column already names every
/// one of them; `None` when a constructor may be missing.
fn complete_signature(heads: &[&Head]) -> Option<Vec<(Head, usize)>> {
    match heads.first()? {
        Head::Tuple(arity) => Some(vec![(Head::Tuple(*arity), *arity)]),
        Head::Named(name, _) => {
            let constructors = family(name)?;
            constructors
                .iter()
                .all(|(constructor, _)| {
                    heads
                        .iter()
                        .any(|head| matches!(head, Head::Named(found, _) if found == constructor))
                })
                .then(|| {
                    constructors
                        .iter()
                        .map(|(constructor, arity)| {
                            (Head::Named((*constructor).into(), *arity), *arity)
                        })
                        .collect()
                })
        }
        Head::Literal(_) => None,
    }
}
