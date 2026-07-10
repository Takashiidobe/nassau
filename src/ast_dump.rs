//! Compact S-expression rendering of the parsed program, for `--dump-ast`.
//!
//! Spans are left out so the output is stable and easy to assert on.

use crate::parser::{Expr, ExprKind, Pat, PatKind, Program, Rule, StmtKind, Ty, TyKind};

pub fn program(program: &Program) -> String {
    if program.statements.is_empty() {
        return format!("(result {})\n", program.result);
    }
    let mut out = String::new();
    for statement in &program.statements {
        out += &match &statement.value {
            StmtKind::Val(name, expr) => format!("(val {name} {})", expr_text(expr)),
            StmtKind::Print(expr) => format!("(print {})", expr_text(expr)),
            StmtKind::Exit(expr) => format!("(exit {})", expr_text(expr)),
        };
        out.push('\n');
    }
    out
}

fn list<'a>(head: &str, parts: impl IntoIterator<Item = &'a String>) -> String {
    let mut out = format!("({head}");
    for part in parts {
        out.push(' ');
        out += part;
    }
    out.push(')');
    out
}

fn expr_text(expr: &Expr) -> String {
    let binary = |name: &str, lhs: &Expr, rhs: &Expr| {
        format!("({name} {} {})", expr_text(lhs), expr_text(rhs))
    };
    match &expr.value {
        ExprKind::Integer(value) => value.to_string(),
        ExprKind::Real(value) => format!("{value:?}"),
        ExprKind::Boolean(value) => value.to_string(),
        ExprKind::Variable(name) => name.clone(),
        ExprKind::String(value) => format!("{value:?}"),
        ExprKind::Character(value) => format!("#{:?}", value.to_string()),
        ExprKind::Word(value) => value.clone(),
        ExprKind::Unit => "()".into(),
        ExprKind::Add(lhs, rhs) => binary("+", lhs, rhs),
        ExprKind::Subtract(lhs, rhs) => binary("-", lhs, rhs),
        ExprKind::Multiply(lhs, rhs) => binary("*", lhs, rhs),
        ExprKind::Divide(lhs, rhs) => binary("/", lhs, rhs),
        ExprKind::IntDivide(lhs, rhs) => binary("div", lhs, rhs),
        ExprKind::Greater(lhs, rhs) => binary(">", lhs, rhs),
        ExprKind::GreaterEqual(lhs, rhs) => binary(">=", lhs, rhs),
        ExprKind::Less(lhs, rhs) => binary("<", lhs, rhs),
        ExprKind::LessEqual(lhs, rhs) => binary("<=", lhs, rhs),
        ExprKind::Equal(lhs, rhs) => binary("=", lhs, rhs),
        ExprKind::NotEqual(lhs, rhs) => binary("<>", lhs, rhs),
        ExprKind::Infix(name, lhs, rhs) => binary(name, lhs, rhs),
        ExprKind::AndAlso(lhs, rhs) => binary("andalso", lhs, rhs),
        ExprKind::OrElse(lhs, rhs) => binary("orelse", lhs, rhs),
        ExprKind::Apply(function, argument) => binary("app", function, argument),
        ExprKind::While(condition, body) => binary("while", condition, body),
        ExprKind::If(condition, consequent, alternative) => format!(
            "(if {} {} {})",
            expr_text(condition),
            expr_text(consequent),
            expr_text(alternative)
        ),
        ExprKind::List(items) => list("list", &items.iter().map(expr_text).collect::<Vec<_>>()),
        ExprKind::Tuple(items) => list("tuple", &items.iter().map(expr_text).collect::<Vec<_>>()),
        ExprKind::Sequence(items) => list("seq", &items.iter().map(expr_text).collect::<Vec<_>>()),
        ExprKind::Record(fields) => list(
            "record",
            &fields
                .iter()
                .map(|(label, value)| format!("({label} {})", expr_text(value)))
                .collect::<Vec<_>>(),
        ),
        ExprKind::Selector(label) => format!("(# {label})"),
        ExprKind::Let(bindings, body) => {
            let bindings = bindings
                .iter()
                .map(|(name, value)| {
                    format!("({} {})", name.as_deref().unwrap_or("_"), expr_text(value))
                })
                .collect::<Vec<_>>();
            format!("(let ({}) {})", bindings.join(" "), expr_text(body))
        }
        ExprKind::Case(scrutinee, rules) => list_with("case", expr_text(scrutinee), rules),
        ExprKind::Fn(rules) => list_with("fn", String::new(), rules),
        ExprKind::Handle(body, rules) => list_with("handle", expr_text(body), rules),
        ExprKind::Raise(inner) => format!("(raise {})", expr_text(inner)),
        ExprKind::Typed(inner, ty) => format!("(: {} {})", expr_text(inner), ty_text(ty)),
        ExprKind::Word8FromInt(inner) => format!("(Word8.fromInt {})", expr_text(inner)),
        ExprKind::PosixExit(inner) => format!("(Posix.Process.exit {})", expr_text(inner)),
    }
}

fn list_with(head: &str, first: String, rules: &[Rule]) -> String {
    let mut parts = Vec::new();
    if !first.is_empty() {
        parts.push(first);
    }
    parts.extend(
        rules
            .iter()
            .map(|(pattern, body)| format!("({} {})", pat_text(pattern), expr_text(body))),
    );
    list(head, &parts)
}

fn pat_text(pattern: &Pat) -> String {
    match &pattern.value {
        PatKind::Wildcard => "_".into(),
        PatKind::Variable(name) => name.clone(),
        PatKind::Integer(value) => value.to_string(),
        PatKind::String(value) => format!("{value:?}"),
        PatKind::Character(value) => format!("#{:?}", value.to_string()),
        PatKind::Boolean(value) => value.to_string(),
        PatKind::Unit => "()".into(),
        PatKind::Tuple(items) => list("tuple", &items.iter().map(pat_text).collect::<Vec<_>>()),
        PatKind::List(items) => list("list", &items.iter().map(pat_text).collect::<Vec<_>>()),
        PatKind::Cons(head, tail) => format!("(:: {} {})", pat_text(head), pat_text(tail)),
        PatKind::Typed(inner, ty) => format!("(: {} {})", pat_text(inner), ty_text(ty)),
    }
}

fn ty_text(ty: &Ty) -> String {
    match &ty.value {
        TyKind::Variable(name) => name.clone(),
        TyKind::Constructor(name, arguments) if arguments.is_empty() => name.clone(),
        TyKind::Constructor(name, arguments) => {
            let mut parts = vec![name.clone()];
            parts.extend(arguments.iter().map(ty_text));
            list("tycon", &parts)
        }
        TyKind::Tuple(items) => list("*", &items.iter().map(ty_text).collect::<Vec<_>>()),
        TyKind::Arrow(from, to) => format!("(-> {} {})", ty_text(from), ty_text(to)),
        TyKind::Record(fields) => list(
            "record",
            &fields
                .iter()
                .map(|(label, ty)| format!("({label} {})", ty_text(ty)))
                .collect::<Vec<_>>(),
        ),
    }
}
