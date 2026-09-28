//! Exhaustiveness and redundancy checking for `case`, `fn` and `handle`.
//!
//! This is the classic pattern-matrix "usefulness" algorithm. It works on the
//! shape of the patterns alone, before types are known, so it learns the
//! constructor families from the declarations in scope (`bool`, `list`,
//! `option`, `order` and every `datatype`, plus single-constructor tuples and
//! records). Constructors of open types, such as exceptions, can be matched
//! redundantly but never exhaustively without a catch-all.
//!
//! As in SML/NJ, a redundant rule is an error and a non-exhaustive match is
//! only a warning (`handle` re-raises, so it is never non-exhaustive). Like
//! SML/NJ, refutable `val` patterns are not reported.

use crate::constructors::Constructors;
use crate::parser::{Decl, DeclKind, ExprKind, Pat, PatKind, Program};
use crate::walk::{Node, walk_program};

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
    let mut out = Vec::new();
    walk_program(program, &mut |node, env| match node {
        Node::Expr(expr) => match &expr.value {
            ExprKind::Case(_, rules) | ExprKind::Fn(rules) => {
                check_rules(
                    rules.iter().map(|(pattern, _)| pattern),
                    (&expr.start, expr.source_span()),
                    true,
                    env,
                    &mut out,
                );
            }
            ExprKind::Handle(_, rules) => {
                check_rules(
                    rules.iter().map(|(pattern, _)| pattern),
                    (&expr.start, expr.source_span()),
                    false,
                    env,
                    &mut out,
                );
            }
            _ => {}
        },
        Node::Decl(declaration) => check_declaration(declaration, env, &mut out),
    });
    out
}

fn check_declaration(declaration: &Decl, env: &Constructors, out: &mut Vec<MatchDiagnostic>) {
    match &declaration.value {
        DeclKind::Fun(bindings) => {
            for binding in bindings {
                let mut labels = Vec::new();
                for clause in &binding.clauses {
                    clause
                        .parameters
                        .iter()
                        .for_each(|p| collect_labels(p, &mut labels));
                }
                sort_labels(&mut labels);
                let rows: Vec<Vec<P>> = binding
                    .clauses
                    .iter()
                    .map(|clause| {
                        clause
                            .parameters
                            .iter()
                            .map(|p| convert(p, &labels, env))
                            .collect()
                    })
                    .collect();
                for (index, clause) in binding.clauses.iter().enumerate() {
                    if !useful(&rows[..index], &rows[index], env) {
                        let first = &clause.parameters[0];
                        out.push(diagnostic(
                            MatchDiagnosticKind::Redundant,
                            &first.start,
                            first.source_span(),
                        ));
                    }
                }
                let arity = rows[0].len();
                if useful(&rows, &wildcards(arity), env) {
                    let first = &binding.clauses[0];
                    let start = &first.parameters[0].start;
                    let last = &binding.clauses[binding.clauses.len() - 1].body;
                    out.push(diagnostic(
                        MatchDiagnosticKind::NonExhaustive,
                        start,
                        (start.offset, last.end.offset - start.offset).into(),
                    ));
                }
            }
        }
        DeclKind::Val { .. }
        | DeclKind::Type(_)
        | DeclKind::Datatype { .. }
        | DeclKind::DatatypeCopy { .. }
        | DeclKind::Abstype { .. }
        | DeclKind::Local(..)
        | DeclKind::Fixity { .. } => {}
    }
}

fn diagnostic(
    kind: MatchDiagnosticKind,
    start: &crate::span::Loc,
    span: miette::SourceSpan,
) -> MatchDiagnostic {
    MatchDiagnostic {
        kind,
        line: start.line,
        column: start.column,
        span,
    }
}

fn check_rules<'a>(
    patterns: impl Iterator<Item = &'a Pat> + Clone,
    whole: (&crate::span::Loc, miette::SourceSpan),
    must_be_exhaustive: bool,
    env: &Constructors,
    out: &mut Vec<MatchDiagnostic>,
) {
    let mut labels = Vec::new();
    for pattern in patterns.clone() {
        collect_labels(pattern, &mut labels);
    }
    sort_labels(&mut labels);
    let patterns: Vec<&Pat> = patterns.collect();
    let rows: Vec<Vec<P>> = patterns
        .iter()
        .map(|pattern| vec![convert(pattern, &labels, env)])
        .collect();

    for (index, pattern) in patterns.iter().enumerate() {
        if !useful(&rows[..index], &rows[index], env) {
            out.push(diagnostic(
                MatchDiagnosticKind::Redundant,
                &pattern.start,
                pattern.source_span(),
            ));
        }
    }
    if must_be_exhaustive && useful(&rows, &[P::Wild], env) {
        out.push(diagnostic(
            MatchDiagnosticKind::NonExhaustive,
            whole.0,
            whole.1,
        ));
    }
}

fn sort_labels(labels: &mut Vec<String>) {
    labels.sort_by(|a, b| label_order(a, b));
    labels.dedup();
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

fn convert(pattern: &Pat, labels: &[String], env: &Constructors) -> P {
    let named = |name: &str, arguments: Vec<P>| {
        P::Con(Head::Named(name.into(), arguments.len()), arguments)
    };
    match &pattern.value {
        PatKind::Wildcard => P::Wild,
        // A nullary constructor in scope, not a variable.
        PatKind::Variable(name) if env.is_constructor(name) => named(name, vec![]),
        PatKind::Variable(_) => P::Wild,
        PatKind::Integer(value) => P::Con(Head::Literal(format!("int {value}")), vec![]),
        PatKind::Word(value) => P::Con(Head::Literal(format!("word {value}")), vec![]),
        PatKind::String(value) => P::Con(Head::Literal(format!("string {value}")), vec![]),
        PatKind::Character(value) => P::Con(Head::Literal(format!("char {value}")), vec![]),
        PatKind::Boolean(value) => named(if *value { "true" } else { "false" }, vec![]),
        PatKind::Unit => P::Con(Head::Tuple(0), vec![]),
        PatKind::Tuple(items) => P::Con(
            Head::Tuple(items.len()),
            items
                .iter()
                .map(|item| convert(item, labels, env))
                .collect(),
        ),
        PatKind::List(items) => items.iter().rev().fold(named("nil", vec![]), |tail, item| {
            named("::", vec![convert(item, labels, env), tail])
        }),
        PatKind::Record(fields, _) => P::Con(
            Head::Tuple(labels.len()),
            labels
                .iter()
                .map(|label| {
                    fields
                        .iter()
                        .find(|(field, _)| field == label)
                        .map_or(P::Wild, |(_, inner)| convert(inner, labels, env))
                })
                .collect(),
        ),
        PatKind::Constructor(name, argument) => named(name, vec![convert(argument, labels, env)]),
        PatKind::Cons(head, tail) => named(
            "::",
            vec![convert(head, labels, env), convert(tail, labels, env)],
        ),
        PatKind::Layered(_, _, inner) | PatKind::Typed(inner, _) => convert(inner, labels, env),
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
fn useful(rows: &[Vec<P>], vector: &[P], env: &Constructors) -> bool {
    let Some((first, rest)) = vector.split_first() else {
        return rows.is_empty();
    };
    match first {
        P::Con(head, arguments) => {
            let arity = arguments.len();
            let specialized = specialize(rows, head, arity);
            let next: Vec<P> = arguments.iter().chain(rest).cloned().collect();
            useful(&specialized, &next, env)
        }
        P::Wild => {
            let heads: Vec<&Head> = rows
                .iter()
                .filter_map(|row| match &row[0] {
                    P::Con(head, _) => Some(head),
                    P::Wild => None,
                })
                .collect();
            match complete_signature(&heads, env) {
                Some(constructors) => constructors.iter().any(|(head, arity)| {
                    let specialized = specialize(rows, head, *arity);
                    let next: Vec<P> = wildcards(*arity)
                        .into_iter()
                        .chain(rest.iter().cloned())
                        .collect();
                    useful(&specialized, &next, env)
                }),
                None => useful(&default_rows(rows), rest, env),
            }
        }
    }
}

/// All constructors of the column's type, when the column already names every
/// one of them; `None` when a constructor may be missing.
fn complete_signature(heads: &[&Head], env: &Constructors) -> Option<Vec<(Head, usize)>> {
    match heads.first()? {
        Head::Tuple(arity) => Some(vec![(Head::Tuple(*arity), *arity)]),
        Head::Named(name, _) => {
            let constructors = env.family_of(name)?;
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
                            (Head::Named(constructor.clone(), *arity), *arity)
                        })
                        .collect()
                })
        }
        Head::Literal(_) => None,
    }
}
