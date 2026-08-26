use std::collections::BTreeSet;

use super::*;
use crate::parser::ExprKind;
use crate::walk::{Node, walk_program};

pub(super) fn roots(declaration: &Decl, environment: &Environment) -> BTreeSet<GlobalId> {
    let mut names = BTreeSet::new();
    let mut structures = BTreeSet::new();
    let mut functors = BTreeSet::new();
    let program = Program {
        statements: vec![crate::span::Span::new(
            declaration.start.clone(),
            declaration.end.clone(),
            StmtKind::Declaration(declaration.clone()),
        )],
        result: 0,
    };
    walk_program(&program, &mut |node, _| match node {
        Node::Expr(expr) => match &expr.value {
            ExprKind::Variable(name) | ExprKind::Infix(name, _, _) => {
                names.insert(name.clone());
            }
            ExprKind::Case(_, rules) | ExprKind::Handle(_, rules) | ExprKind::Fn(rules) => {
                for (pat, _) in rules {
                    pattern(pat, &mut names);
                }
            }
            _ => {}
        },
        Node::Decl(decl) => match &decl.value {
            DeclKind::Val { bindings, .. } => {
                for (pat, _) in bindings {
                    pattern(pat, &mut names);
                }
            }
            DeclKind::Fun(bindings) => {
                for binding in bindings {
                    for clause in &binding.clauses {
                        for pat in &clause.parameters {
                            pattern(pat, &mut names);
                        }
                    }
                }
            }
            DeclKind::Exception(bindings) => {
                for binding in bindings {
                    if let ExceptionKind::Copy(name) = &binding.kind {
                        names.insert(name.clone());
                    }
                }
            }
            DeclKind::Open(paths) => structures.extend(paths.iter().cloned()),
            DeclKind::Structure(bindings) => {
                for binding in bindings {
                    structure(&binding.body, &mut structures, &mut functors);
                }
            }
            DeclKind::Functor(bindings) => {
                for binding in bindings {
                    structure(&binding.body, &mut structures, &mut functors);
                }
            }
            _ => {}
        },
    });
    structures.extend(
        names
            .iter()
            .filter_map(|name| name.rsplit_once('.').map(|(path, _)| path.to_owned())),
    );
    let mut roots = BTreeSet::new();
    let mut seen = BTreeSet::new();
    for (name, binding) in environment.globals.iter().rev() {
        if !seen.insert(name) {
            continue;
        }
        let base = name.strip_prefix("exn ").unwrap_or(name);
        if names.contains(base)
            && let Binding::Global(global, _) = binding
        {
            roots.insert(*global);
        }
    }
    seen.clear();
    for (name, value) in environment.structures.iter().rev() {
        if !seen.insert(name) {
            continue;
        }
        if structures
            .iter()
            .any(|path| path.split('.').next() == Some(name))
        {
            environment_roots(&value.values, &value.structures, &[], &mut roots);
        }
    }
    seen.clear();
    for (name, functor) in environment.functors.iter().rev() {
        if !seen.insert(name) {
            continue;
        }
        if functors.contains(name) {
            roots.extend(&functor.roots);
        }
    }
    roots
}

fn pattern(pat: &Pat, names: &mut BTreeSet<String>) {
    match &pat.value {
        PatKind::Variable(name) => {
            names.insert(name.clone());
        }
        PatKind::Constructor(name, inner) => {
            names.insert(name.clone());
            pattern(inner, names);
        }
        PatKind::Tuple(items) | PatKind::List(items) => {
            for item in items {
                pattern(item, names);
            }
        }
        PatKind::Record(fields, _) => {
            for (_, item) in fields {
                pattern(item, names);
            }
        }
        PatKind::Cons(head, tail) => {
            pattern(head, names);
            pattern(tail, names);
        }
        PatKind::Layered(_, _, inner) | PatKind::Typed(inner, _) => pattern(inner, names),
        _ => {}
    }
}

fn structure(exp: &StrExp, structures: &mut BTreeSet<String>, functors: &mut BTreeSet<String>) {
    match &exp.value {
        StrExpKind::Name(name) => {
            structures.insert(name.clone());
        }
        StrExpKind::Apply(name, argument) => {
            functors.insert(name.clone());
            structure(argument, structures, functors);
        }
        StrExpKind::Ascribed { body, .. } | StrExpKind::Let(_, body) => {
            structure(body, structures, functors)
        }
        StrExpKind::Struct(_) => {}
    }
}
