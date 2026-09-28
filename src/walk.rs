//! A single pre-order traversal of every declaration and expression in a
//! program, shared by the static checks.

use crate::parser::{Decl, DeclKind, Expr, ExprKind, Program, StmtKind};

#[derive(Clone, Copy)]
pub enum Node<'a> {
    Expr(&'a Expr),
    Decl(&'a Decl),
}

pub fn walk_program<'a>(program: &'a Program, visit: &mut dyn FnMut(Node<'a>)) {
    for statement in &program.statements {
        match &statement.value {
            StmtKind::Val(_, expr) | StmtKind::Print(expr) | StmtKind::Exit(expr) => {
                walk_expr(expr, visit);
            }
            StmtKind::Declaration(declaration) => walk_decl(declaration, visit),
        }
    }
}

fn walk_decl<'a>(declaration: &'a Decl, visit: &mut dyn FnMut(Node<'a>)) {
    visit(Node::Decl(declaration));
    match &declaration.value {
        DeclKind::Val { bindings, .. } => {
            for (_, expr) in bindings {
                walk_expr(expr, visit);
            }
        }
        DeclKind::Fun(bindings) => {
            for binding in bindings {
                for clause in &binding.clauses {
                    walk_expr(&clause.body, visit);
                }
            }
        }
        DeclKind::Local(private, public) => {
            for inner in private.iter().chain(public) {
                walk_decl(inner, visit);
            }
        }
        DeclKind::Type(_) | DeclKind::Fixity { .. } => {}
    }
}

fn walk_expr<'a>(expr: &'a Expr, visit: &mut dyn FnMut(Node<'a>)) {
    visit(Node::Expr(expr));
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
            walk_expr(lhs, visit);
            walk_expr(rhs, visit);
        }
        ExprKind::If(condition, consequent, alternative) => {
            walk_expr(condition, visit);
            walk_expr(consequent, visit);
            walk_expr(alternative, visit);
        }
        ExprKind::List(items) | ExprKind::Tuple(items) | ExprKind::Sequence(items) => {
            items.iter().for_each(|item| walk_expr(item, visit));
        }
        ExprKind::Record(fields) => fields.iter().for_each(|(_, value)| walk_expr(value, visit)),
        ExprKind::Word8FromInt(inner)
        | ExprKind::PosixExit(inner)
        | ExprKind::Raise(inner)
        | ExprKind::Typed(inner, _) => walk_expr(inner, visit),
        ExprKind::Let(declarations, body) => {
            declarations
                .iter()
                .for_each(|inner| walk_decl(inner, visit));
            walk_expr(body, visit);
        }
        ExprKind::Case(scrutinee, rules) | ExprKind::Handle(scrutinee, rules) => {
            walk_expr(scrutinee, visit);
            rules.iter().for_each(|(_, body)| walk_expr(body, visit));
        }
        ExprKind::Fn(rules) => rules.iter().for_each(|(_, body)| walk_expr(body, visit)),
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
