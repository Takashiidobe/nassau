//! Static checks on what declarations and patterns bind.
//!
//! The Definition requires every variable in a pattern (and across the
//! patterns of one `val ... and ...` group) to be distinct, every function in
//! one `fun ... and ...` group to have a distinct name, and every variable in
//! the parameters of one `fun` clause to be distinct. Unbound names are
//! reported by type checking, which has the environment to resolve them.

use crate::error::ScopeErrorKind;
use crate::matching::looks_like_constructor;
use crate::parser::{DeclKind, ExprKind, Pat, PatKind, Program};
use crate::walk::{Node, walk_program};

pub struct ScopeDiagnostic {
    pub kind: ScopeErrorKind,
    pub span: miette::SourceSpan,
}

pub fn check_program(program: &Program) -> Vec<ScopeDiagnostic> {
    let mut out = Vec::new();
    walk_program(program, &mut |node| match node {
        Node::Decl(declaration) => match &declaration.value {
            DeclKind::Val { bindings, .. } => {
                check_distinct(bindings.iter().map(|(pattern, _)| pattern), &mut out);
            }
            DeclKind::Fun(bindings) => {
                let mut seen: Vec<&str> = Vec::new();
                for binding in bindings {
                    if seen.contains(&binding.name.as_str()) {
                        out.push(ScopeDiagnostic {
                            kind: ScopeErrorKind::DuplicateFunction(binding.name.clone()),
                            span: declaration.source_span(),
                        });
                    }
                    seen.push(&binding.name);
                    for clause in &binding.clauses {
                        check_distinct(clause.parameters.iter(), &mut out);
                    }
                }
            }
            _ => {}
        },
        Node::Expr(expr) => match &expr.value {
            ExprKind::Fn(rules) | ExprKind::Case(_, rules) | ExprKind::Handle(_, rules) => {
                for (pattern, _) in rules {
                    check_distinct(std::iter::once(pattern), &mut out);
                }
            }
            _ => {}
        },
    });
    out
}

/// Reports each variable bound more than once across `patterns`.
fn check_distinct<'a>(patterns: impl Iterator<Item = &'a Pat>, out: &mut Vec<ScopeDiagnostic>) {
    let mut seen: Vec<String> = Vec::new();
    for pattern in patterns {
        let mut variables = Vec::new();
        pattern_variables(pattern, &mut variables);
        for (name, span) in variables {
            if seen.contains(&name) {
                out.push(ScopeDiagnostic {
                    kind: ScopeErrorKind::DuplicateVariable(name),
                    span,
                });
            } else {
                seen.push(name);
            }
        }
    }
}

/// The variables a pattern binds, with the span of each occurrence.
pub fn pattern_variables(pattern: &Pat, out: &mut Vec<(String, miette::SourceSpan)>) {
    match &pattern.value {
        PatKind::Variable(name) => {
            if !looks_like_constructor(name) {
                out.push((name.clone(), pattern.source_span()));
            }
        }
        PatKind::Layered(name, _, inner) => {
            out.push((name.clone(), pattern.source_span()));
            pattern_variables(inner, out);
        }
        PatKind::Tuple(items) | PatKind::List(items) => {
            items.iter().for_each(|item| pattern_variables(item, out));
        }
        PatKind::Record(fields, _) => {
            fields
                .iter()
                .for_each(|(_, inner)| pattern_variables(inner, out));
        }
        PatKind::Constructor(_, inner) | PatKind::Typed(inner, _) => pattern_variables(inner, out),
        PatKind::Cons(head, tail) => {
            pattern_variables(head, out);
            pattern_variables(tail, out);
        }
        PatKind::Wildcard
        | PatKind::Integer(_)
        | PatKind::Word(_)
        | PatKind::String(_)
        | PatKind::Character(_)
        | PatKind::Boolean(_)
        | PatKind::Unit => {}
    }
}
