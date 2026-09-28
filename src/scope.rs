//! Static checks on what declarations and patterns bind.
//!
//! The Definition requires every variable in a pattern (and across the
//! patterns of one `val ... and ...` group) to be distinct, every function in
//! one `fun ... and ...` group to have a distinct name, and every variable in
//! the parameters of one `fun` clause to be distinct. Unbound names are
//! reported by type checking, which has the environment to resolve them.

use crate::constructors::Constructors;
use crate::error::ScopeErrorKind;
use crate::parser::{DeclKind, ExprKind, Pat, PatKind, Program};
use crate::walk::{Node, walk_program};

pub struct ScopeDiagnostic {
    pub kind: ScopeErrorKind,
    pub span: miette::SourceSpan,
}

pub fn check_program(program: &Program) -> Vec<ScopeDiagnostic> {
    let mut out = Vec::new();
    walk_program(program, &mut |node, env| match node {
        Node::Decl(declaration) => match &declaration.value {
            DeclKind::Val { bindings, .. } => {
                check_distinct(bindings.iter().map(|(pattern, _)| pattern), env, &mut out);
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
                        check_distinct(clause.parameters.iter(), env, &mut out);
                    }
                }
            }
            DeclKind::Datatype {
                bindings, withtype, ..
            }
            | DeclKind::Abstype {
                bindings, withtype, ..
            } => {
                let mut types: Vec<&str> = Vec::new();
                let mut constructors: Vec<&str> = Vec::new();
                let names = bindings
                    .iter()
                    .map(|binding| binding.name.as_str())
                    .chain(withtype.iter().map(|binding| binding.name.as_str()));
                for name in names {
                    if types.contains(&name) {
                        out.push(ScopeDiagnostic {
                            kind: ScopeErrorKind::DuplicateType(name.into()),
                            span: declaration.source_span(),
                        });
                    }
                    types.push(name);
                }
                for constructor in bindings.iter().flat_map(|binding| &binding.constructors) {
                    let name = constructor.value.name.as_str();
                    if constructors.contains(&name) {
                        out.push(ScopeDiagnostic {
                            kind: ScopeErrorKind::DuplicateConstructor(name.into()),
                            span: constructor.source_span(),
                        });
                    }
                    constructors.push(name);
                }
            }
            DeclKind::Exception(bindings) => {
                let mut seen: Vec<&str> = Vec::new();
                for binding in bindings {
                    if seen.contains(&binding.name.as_str()) {
                        out.push(ScopeDiagnostic {
                            kind: ScopeErrorKind::DuplicateException(binding.name.clone()),
                            span: declaration.source_span(),
                        });
                    }
                    seen.push(&binding.name);
                }
            }
            DeclKind::Type(bindings) => {
                let mut seen: Vec<&str> = Vec::new();
                for binding in bindings {
                    if seen.contains(&binding.name.as_str()) {
                        out.push(ScopeDiagnostic {
                            kind: ScopeErrorKind::DuplicateType(binding.name.clone()),
                            span: declaration.source_span(),
                        });
                    }
                    seen.push(&binding.name);
                }
            }
            _ => {}
        },
        Node::Expr(expr) => match &expr.value {
            ExprKind::Fn(rules) | ExprKind::Case(_, rules) | ExprKind::Handle(_, rules) => {
                for (pattern, _) in rules {
                    check_distinct(std::iter::once(pattern), env, &mut out);
                }
            }
            _ => {}
        },
    });
    out
}

/// Reports each variable bound more than once across `patterns`.
fn check_distinct<'a>(
    patterns: impl Iterator<Item = &'a Pat>,
    env: &Constructors,
    out: &mut Vec<ScopeDiagnostic>,
) {
    let mut seen: Vec<String> = Vec::new();
    for pattern in patterns {
        let mut variables = Vec::new();
        pattern_variables(pattern, env, &mut variables);
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
pub fn pattern_variables(
    pattern: &Pat,
    env: &Constructors,
    out: &mut Vec<(String, miette::SourceSpan)>,
) {
    match &pattern.value {
        PatKind::Variable(name) => {
            if !env.is_constructor(name) {
                out.push((name.clone(), pattern.source_span()));
            }
        }
        PatKind::Layered(name, _, inner) => {
            out.push((name.clone(), pattern.source_span()));
            pattern_variables(inner, env, out);
        }
        PatKind::Tuple(items) | PatKind::List(items) => {
            items
                .iter()
                .for_each(|item| pattern_variables(item, env, out));
        }
        PatKind::Record(fields, _) => {
            fields
                .iter()
                .for_each(|(_, inner)| pattern_variables(inner, env, out));
        }
        PatKind::Constructor(_, inner) | PatKind::Typed(inner, _) => {
            pattern_variables(inner, env, out)
        }
        PatKind::Cons(head, tail) => {
            pattern_variables(head, env, out);
            pattern_variables(tail, env, out);
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
