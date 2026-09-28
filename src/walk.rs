//! A single pre-order traversal of every declaration and expression in a
//! program, shared by the static checks. Each visit also gets the constructors
//! in scope at that point.

use crate::constructors::Constructors;
use crate::parser::{Decl, DeclKind, Expr, ExprKind, Program, StmtKind};

#[derive(Clone, Copy)]
pub enum Node<'a> {
    Expr(&'a Expr),
    Decl(&'a Decl),
}

pub fn walk_program<'a>(program: &'a Program, visit: &mut dyn FnMut(Node<'a>, &Constructors)) {
    let mut env = Constructors::new();
    for statement in &program.statements {
        match &statement.value {
            StmtKind::Val(_, expr) | StmtKind::Print(expr) | StmtKind::Exit(expr) => {
                walk_expr(expr, visit, &mut env);
            }
            StmtKind::Declaration(declaration) => walk_decl(declaration, visit, &mut env),
        }
    }
}

fn walk_decl<'a>(
    declaration: &'a Decl,
    visit: &mut dyn FnMut(Node<'a>, &Constructors),
    env: &mut Constructors,
) {
    visit(Node::Decl(declaration), env);
    match &declaration.value {
        DeclKind::Val { bindings, .. } => {
            for (_, expr) in bindings {
                walk_expr(expr, visit, env);
            }
        }
        DeclKind::Fun(bindings) => {
            for binding in bindings {
                for clause in &binding.clauses {
                    walk_expr(&clause.body, visit, env);
                }
            }
        }
        DeclKind::Local(private, public) => {
            let outer = env.mark();
            for inner in private {
                walk_decl(inner, visit, env);
            }
            let visible = env.mark();
            for inner in public {
                walk_decl(inner, visit, env);
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
                walk_decl(inner, visit, env);
            }
            env.hide_constructors(before, after);
        }
        DeclKind::Datatype { .. } | DeclKind::DatatypeCopy { .. } | DeclKind::Exception(_) => {
            env.declare(&declaration.value);
        }
        DeclKind::Type(_) | DeclKind::Fixity { .. } => {}
    }
}

fn walk_expr<'a>(
    expr: &'a Expr,
    visit: &mut dyn FnMut(Node<'a>, &Constructors),
    env: &mut Constructors,
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
            walk_expr(lhs, visit, env);
            walk_expr(rhs, visit, env);
        }
        ExprKind::If(condition, consequent, alternative) => {
            walk_expr(condition, visit, env);
            walk_expr(consequent, visit, env);
            walk_expr(alternative, visit, env);
        }
        ExprKind::List(items) | ExprKind::Tuple(items) | ExprKind::Sequence(items) => {
            for item in items {
                walk_expr(item, visit, env);
            }
        }
        ExprKind::Record(fields) => {
            for (_, value) in fields {
                walk_expr(value, visit, env);
            }
        }
        ExprKind::Word8FromInt(inner)
        | ExprKind::PosixExit(inner)
        | ExprKind::Raise(inner)
        | ExprKind::Typed(inner, _) => walk_expr(inner, visit, env),
        ExprKind::Let(declarations, body) => {
            let mark = env.mark();
            for inner in declarations {
                walk_decl(inner, visit, env);
            }
            walk_expr(body, visit, env);
            env.release(mark);
        }
        ExprKind::Case(scrutinee, rules) | ExprKind::Handle(scrutinee, rules) => {
            walk_expr(scrutinee, visit, env);
            for (_, body) in rules {
                walk_expr(body, visit, env);
            }
        }
        ExprKind::Fn(rules) => {
            for (_, body) in rules {
                walk_expr(body, visit, env);
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
