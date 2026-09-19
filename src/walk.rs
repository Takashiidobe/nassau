//! A single pre-order traversal of every declaration and expression in a
//! program, shared by the static checks. Each visit also gets the constructors
//! in scope at that point.

use std::collections::HashSet;

use crate::constructors::{Constructors, Declared};
use crate::parser::{
    Decl, DeclKind, Expr, ExprKind, FunctorParameter, Program, SigExp, SigExpKind, SpecKind,
    StmtKind, StrExp, StrExpKind,
};

/// The signatures and functors declared so far, latest last. A functor is
/// kept as the constructors its body declares, which each application adds.
#[derive(Default)]
struct Signatures<'a> {
    signatures: Vec<(&'a str, &'a SigExp)>,
    functors: Vec<(&'a str, Declared)>,
}

#[derive(Clone, Copy)]
pub enum Node<'a> {
    Expr(&'a Expr),
    Decl(&'a Decl),
}

pub fn walk_program<'a>(program: &'a Program, visit: &mut dyn FnMut(Node<'a>, &Constructors)) {
    let mut env = Constructors::new();
    let mut sigs = Signatures::default();
    for statement in &program.statements {
        match &statement.value {
            StmtKind::Val(_, expr) => {
                walk_expr(expr, visit, &mut env, &mut sigs);
            }
            StmtKind::Declaration(declaration) => {
                walk_decl(declaration, visit, &mut env, &mut sigs)
            }
        }
    }
}

fn walk_decl<'a>(
    declaration: &'a Decl,
    visit: &mut dyn FnMut(Node<'a>, &Constructors),
    env: &mut Constructors,
    sigs: &mut Signatures<'a>,
) {
    visit(Node::Decl(declaration), env);
    match &declaration.value {
        DeclKind::Val { bindings, .. } => {
            for (_, expr) in bindings {
                walk_expr(expr, visit, env, sigs);
            }
        }
        DeclKind::Fun(bindings) => {
            for binding in bindings {
                for clause in &binding.clauses {
                    walk_expr(&clause.body, visit, env, sigs);
                }
            }
        }
        DeclKind::Local(private, public) => {
            let outer = env.mark();
            for inner in private {
                walk_decl(inner, visit, env, sigs);
            }
            let visible = env.mark();
            for inner in public {
                walk_decl(inner, visit, env, sigs);
            }
            // Only the public part outlives the declaration.
            let kept = env.take_since(visible);
            env.release(outer);
            env.restore(kept);
        }
        DeclKind::Abstype { bindings, body, .. } => {
            let before = env.mark();
            env.declare_datatypes(bindings);
            let after = env.mark();
            for inner in body {
                walk_decl(inner, visit, env, sigs);
            }
            env.hide_constructors(before, after);
        }
        DeclKind::Datatype { .. } | DeclKind::DatatypeCopy { .. } | DeclKind::Exception(_) => {
            env.declare(&declaration.value);
        }
        DeclKind::Structure(bindings) => {
            for binding in bindings {
                let mark = env.mark();
                walk_strexp(&binding.body, visit, env, sigs);
                env.qualify_since(mark, &binding.name);
            }
        }
        DeclKind::Signature(bindings) => {
            for binding in bindings {
                sigs.signatures.push((&binding.name, &binding.body));
            }
        }
        DeclKind::Functor(bindings) => {
            for binding in bindings {
                let outer = env.mark();
                match &binding.parameter {
                    FunctorParameter::Named(name, signature) => {
                        declare_signature(signature, sigs, env);
                        env.qualify_since(outer, name);
                    }
                    FunctorParameter::Specs(signature) => declare_signature(signature, sigs, env),
                }
                let inner = env.mark();
                walk_strexp(&binding.body, visit, env, sigs);
                let result = env.take_since(inner);
                env.release(outer);
                sigs.functors.push((&binding.name, result));
            }
        }
        DeclKind::Open(paths) => {
            for path in paths {
                env.open(path);
            }
        }
        DeclKind::Type(_) | DeclKind::Fixity { .. } => {}
    }
}

/// Declares the constructors a structure expression provides, unqualified.
fn walk_strexp<'a>(
    body: &'a StrExp,
    visit: &mut dyn FnMut(Node<'a>, &Constructors),
    env: &mut Constructors,
    sigs: &mut Signatures<'a>,
) {
    match &body.value {
        StrExpKind::Struct(declarations) => {
            for declaration in declarations {
                walk_decl(declaration, visit, env, sigs);
            }
        }
        StrExpKind::Name(path) => env.open(path),
        StrExpKind::Apply(name, argument) => {
            let mark = env.mark();
            walk_strexp(argument, visit, env, sigs);
            env.release(mark);
            let found = sigs
                .functors
                .iter()
                .rev()
                .find(|(known, _)| known == name)
                .map(|(_, result)| result.clone());
            if let Some(result) = found {
                env.restore(result);
            }
        }
        StrExpKind::Ascribed {
            body, signature, ..
        } => {
            let mark = env.mark();
            walk_strexp(body, visit, env, sigs);
            let mut visible = HashSet::new();
            signature_constructors(signature, sigs, "", &mut visible);
            env.retain_since(mark, &visible);
        }
        StrExpKind::Let(declarations, inner) => {
            let outer = env.mark();
            for declaration in declarations {
                walk_decl(declaration, visit, env, sigs);
            }
            let visible = env.mark();
            walk_strexp(inner, visit, env, sigs);
            let kept = env.take_since(visible);
            env.release(outer);
            env.restore(kept);
        }
    }
}

/// The names of the constructors a signature specifies, qualified by `prefix`.
fn signature_constructors(
    signature: &SigExp,
    sigs: &Signatures,
    prefix: &str,
    out: &mut HashSet<String>,
) {
    match &signature.value {
        SigExpKind::Name(name) => {
            if let Some((_, found)) = sigs
                .signatures
                .iter()
                .rev()
                .find(|(known, _)| known == name)
            {
                signature_constructors(found, sigs, prefix, out);
            }
        }
        SigExpKind::Where(inner, _) => signature_constructors(inner, sigs, prefix, out),
        SigExpKind::Sig(specs) => {
            for spec in specs {
                match &spec.value {
                    SpecKind::Datatype(bindings) => {
                        for binding in bindings {
                            for constructor in &binding.constructors {
                                out.insert(format!("{prefix}{}", constructor.value.name));
                            }
                        }
                    }
                    SpecKind::Exception(exceptions) => {
                        for (name, _) in exceptions {
                            out.insert(format!("{prefix}{name}"));
                        }
                    }
                    SpecKind::Include(inner) => signature_constructors(inner, sigs, prefix, out),
                    SpecKind::Structure(structures) => {
                        for (name, inner) in structures {
                            signature_constructors(inner, sigs, &format!("{prefix}{name}."), out);
                        }
                    }
                    // A copy's constructors come from the original, which is
                    // not tracked; keep them all visible.
                    SpecKind::DatatypeCopy { .. }
                    | SpecKind::Val(_)
                    | SpecKind::Type(_)
                    | SpecKind::Sharing(_)
                    | SpecKind::SharingStructures(_) => {}
                }
            }
        }
    }
}

fn walk_expr<'a>(
    expr: &'a Expr,
    visit: &mut dyn FnMut(Node<'a>, &Constructors),
    env: &mut Constructors,
    sigs: &mut Signatures<'a>,
) {
    visit(Node::Expr(expr), env);
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
            walk_expr(lhs, visit, env, sigs);
            walk_expr(rhs, visit, env, sigs);
        }
        ExprKind::If(condition, consequent, alternative) => {
            walk_expr(condition, visit, env, sigs);
            walk_expr(consequent, visit, env, sigs);
            walk_expr(alternative, visit, env, sigs);
        }
        ExprKind::List(items) | ExprKind::Tuple(items) | ExprKind::Sequence(items) => {
            for item in items {
                walk_expr(item, visit, env, sigs);
            }
        }
        ExprKind::Record(fields) => {
            for (_, value) in fields {
                walk_expr(value, visit, env, sigs);
            }
        }
        ExprKind::Raise(inner) | ExprKind::Typed(inner, _) => walk_expr(inner, visit, env, sigs),
        ExprKind::Let(declarations, body) => {
            let mark = env.mark();
            for inner in declarations {
                walk_decl(inner, visit, env, sigs);
            }
            walk_expr(body, visit, env, sigs);
            env.release(mark);
        }
        ExprKind::Case(scrutinee, rules) | ExprKind::Handle(scrutinee, rules) => {
            walk_expr(scrutinee, visit, env, sigs);
            for (_, body) in rules {
                walk_expr(body, visit, env, sigs);
            }
        }
        ExprKind::Fn(rules) => {
            for (_, body) in rules {
                walk_expr(body, visit, env, sigs);
            }
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

/// Declares the constructors a functor parameter's signature specifies.
fn declare_signature(signature: &SigExp, sigs: &Signatures, env: &mut Constructors) {
    match &signature.value {
        SigExpKind::Name(name) => {
            if let Some((_, found)) = sigs
                .signatures
                .iter()
                .rev()
                .find(|(known, _)| known == name)
            {
                declare_signature(found, sigs, env);
            }
        }
        SigExpKind::Where(inner, _) => declare_signature(inner, sigs, env),
        SigExpKind::Sig(specs) => {
            for spec in specs {
                match &spec.value {
                    SpecKind::Datatype(bindings) => env.declare_datatypes(bindings),
                    SpecKind::Exception(exceptions) => {
                        env.declare_exceptions(exceptions.iter().map(|(name, _)| name.as_str()))
                    }
                    SpecKind::Include(inner) => declare_signature(inner, sigs, env),
                    SpecKind::Structure(structures) => {
                        for (name, inner) in structures {
                            let mark = env.mark();
                            declare_signature(inner, sigs, env);
                            env.qualify_since(mark, name);
                        }
                    }
                    SpecKind::DatatypeCopy { name, original } => env.declare_copy(name, original),
                    SpecKind::Val(_)
                    | SpecKind::Type(_)
                    | SpecKind::Sharing(_)
                    | SpecKind::SharingStructures(_) => {}
                }
            }
        }
    }
}
