//! Compact S-expression rendering of the parsed program, for `--dump-ast`.
//!
//! Spans are left out so the output is stable and easy to assert on.

use crate::parser::{
    DataBinding, Decl, DeclKind, ExceptionKind, Expr, ExprKind, FixityKind, FunctorParameter, Pat,
    PatKind, Program, Rule, SigExp, SigExpKind, Spec, SpecKind, StmtKind, StrExp, StrExpKind, Ty,
    TyKind,
};

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
            StmtKind::Declaration(declaration) => decl_text(declaration),
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
        ExprKind::Let(declarations, body) => {
            let declarations = declarations.iter().map(decl_text).collect::<Vec<_>>();
            format!("(let ({}) {})", declarations.join(" "), expr_text(body))
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

fn decl_text(declaration: &Decl) -> String {
    let group = |bindings: &[(Pat, Expr)]| {
        bindings
            .iter()
            .map(|(pattern, expr)| format!("({} {})", pat_text(pattern), expr_text(expr)))
            .collect::<Vec<_>>()
    };
    match &declaration.value {
        DeclKind::Val {
            recursive: false,
            bindings,
        } if bindings.len() == 1 => format!(
            "(val {} {})",
            pat_text(&bindings[0].0),
            expr_text(&bindings[0].1)
        ),
        DeclKind::Val {
            recursive,
            bindings,
        } => list(
            if *recursive { "val-rec" } else { "val-and" },
            &group(bindings),
        ),
        DeclKind::Fun(bindings) => {
            let bindings = bindings
                .iter()
                .map(|binding| {
                    let clauses = binding
                        .clauses
                        .iter()
                        .map(|clause| {
                            let parameters: Vec<String> =
                                clause.parameters.iter().map(pat_text).collect();
                            format!("(({}) {})", parameters.join(" "), expr_text(&clause.body))
                        })
                        .collect::<Vec<_>>();
                    format!("({} {})", binding.name, clauses.join(" "))
                })
                .collect::<Vec<_>>();
            list("fun", &bindings)
        }
        DeclKind::Type(bindings) => list("type", &type_bindings(bindings)),
        DeclKind::Datatype { bindings, withtype } => {
            let mut parts = data_bindings(bindings);
            if !withtype.is_empty() {
                parts.push(list("withtype", &type_bindings(withtype)));
            }
            list("datatype", &parts)
        }
        DeclKind::DatatypeCopy { name, original } => format!("(datatype-copy {name} {original})"),
        DeclKind::Exception(bindings) => {
            let bindings = bindings
                .iter()
                .map(|binding| match &binding.kind {
                    ExceptionKind::Fresh(Some(ty)) => format!("({} {})", binding.name, ty_text(ty)),
                    ExceptionKind::Fresh(None) => format!("({})", binding.name),
                    ExceptionKind::Copy(original) => format!("({} = {original})", binding.name),
                })
                .collect::<Vec<_>>();
            list("exception", &bindings)
        }
        DeclKind::Abstype {
            bindings,
            withtype,
            body,
        } => {
            let mut parts = data_bindings(bindings);
            if !withtype.is_empty() {
                parts.push(list("withtype", &type_bindings(withtype)));
            }
            parts.push(list(
                "with",
                &body.iter().map(decl_text).collect::<Vec<_>>(),
            ));
            list("abstype", &parts)
        }
        DeclKind::Structure(bindings) => {
            let bindings: Vec<String> = bindings
                .iter()
                .map(|binding| format!("({} {})", binding.name, strexp_text(&binding.body)))
                .collect();
            list("structure", &bindings)
        }
        DeclKind::Signature(bindings) => {
            let bindings: Vec<String> = bindings
                .iter()
                .map(|binding| format!("({} {})", binding.name, sigexp_text(&binding.body)))
                .collect();
            list("signature", &bindings)
        }
        DeclKind::Functor(bindings) => {
            let bindings: Vec<String> = bindings
                .iter()
                .map(|binding| {
                    let parameter = match &binding.parameter {
                        FunctorParameter::Named(name, signature) => {
                            format!("({name} {})", sigexp_text(signature))
                        }
                        FunctorParameter::Specs(signature) => sigexp_text(signature),
                    };
                    format!(
                        "({} {parameter} {})",
                        binding.name,
                        strexp_text(&binding.body)
                    )
                })
                .collect();
            list("functor", &bindings)
        }
        DeclKind::Open(names) => format!("(open {})", names.join(" ")),
        DeclKind::Local(private, public) => {
            let text = |declarations: &[Decl]| {
                declarations
                    .iter()
                    .map(decl_text)
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            format!("(local ({}) ({}))", text(private), text(public))
        }
        DeclKind::Fixity {
            kind,
            precedence,
            names,
        } => match kind {
            FixityKind::Nonfix => format!("(nonfix {})", names.join(" ")),
            FixityKind::Infix => format!("(infix {precedence} {})", names.join(" ")),
            FixityKind::Infixr => format!("(infixr {precedence} {})", names.join(" ")),
        },
    }
}

fn strexp_text(body: &StrExp) -> String {
    match &body.value {
        StrExpKind::Struct(declarations) => list(
            "struct",
            &declarations.iter().map(decl_text).collect::<Vec<_>>(),
        ),
        StrExpKind::Name(name) => name.clone(),
        StrExpKind::Ascribed {
            body,
            signature,
            opaque,
        } => format!(
            "({} {} {})",
            if *opaque { ":>" } else { ":" },
            strexp_text(body),
            sigexp_text(signature)
        ),
        StrExpKind::Let(declarations, inner) => format!(
            "(let ({}) {})",
            declarations
                .iter()
                .map(decl_text)
                .collect::<Vec<_>>()
                .join(" "),
            strexp_text(inner)
        ),
        StrExpKind::Apply(name, argument) => format!("(app {name} {})", strexp_text(argument)),
    }
}

fn sigexp_text(body: &SigExp) -> String {
    match &body.value {
        SigExpKind::Sig(specs) => list("sig", &specs.iter().map(spec_text).collect::<Vec<_>>()),
        SigExpKind::Name(name) => name.clone(),
        SigExpKind::Where(inner, refinements) => {
            let mut parts = vec![sigexp_text(inner)];
            parts.extend(refinements.iter().map(|refinement| {
                format!(
                    "(type ({}) {} {})",
                    refinement.parameters.join(" "),
                    refinement.name,
                    ty_text(&refinement.ty)
                )
            }));
            list("where", &parts)
        }
    }
}

fn spec_text(spec: &Spec) -> String {
    match &spec.value {
        SpecKind::Val(values) => list(
            "val",
            &values
                .iter()
                .map(|(name, ty)| format!("({name} {})", ty_text(ty)))
                .collect::<Vec<_>>(),
        ),
        SpecKind::Type(specs) => list(
            "type",
            &specs
                .iter()
                .map(|spec| {
                    let head = if spec.equality { "eq " } else { "" };
                    match &spec.definition {
                        Some(ty) => format!(
                            "({head}{} ({}) {})",
                            spec.name,
                            spec.parameters.join(" "),
                            ty_text(ty)
                        ),
                        None => {
                            format!("({head}{} ({}))", spec.name, spec.parameters.join(" "))
                        }
                    }
                })
                .collect::<Vec<_>>(),
        ),
        SpecKind::Datatype(bindings) => list("datatype", &data_bindings(bindings)),
        SpecKind::DatatypeCopy { name, original } => {
            format!("(datatype-copy {name} {original})")
        }
        SpecKind::Exception(exceptions) => list(
            "exception",
            &exceptions
                .iter()
                .map(|(name, argument)| match argument {
                    Some(ty) => format!("({name} {})", ty_text(ty)),
                    None => format!("({name})"),
                })
                .collect::<Vec<_>>(),
        ),
        SpecKind::Structure(structures) => list(
            "structure",
            &structures
                .iter()
                .map(|(name, signature)| format!("({name} {})", sigexp_text(signature)))
                .collect::<Vec<_>>(),
        ),
        SpecKind::Include(signature) => format!("(include {})", sigexp_text(signature)),
        SpecKind::Sharing(names) => format!("(sharing {})", names.join(" ")),
        SpecKind::SharingStructures(names) => format!("(sharing-structures {})", names.join(" ")),
    }
}

fn type_bindings(bindings: &[crate::parser::TypeBinding]) -> Vec<String> {
    bindings
        .iter()
        .map(|binding| {
            format!(
                "({} ({}) {})",
                binding.name,
                binding.parameters.join(" "),
                ty_text(&binding.ty)
            )
        })
        .collect()
}

fn data_bindings(bindings: &[DataBinding]) -> Vec<String> {
    bindings
        .iter()
        .map(|binding| {
            let mut parts = vec![format!("({})", binding.parameters.join(" "))];
            for constructor in &binding.constructors {
                parts.push(match &constructor.value.argument {
                    Some(argument) => format!("({} {})", constructor.value.name, ty_text(argument)),
                    None => format!("({})", constructor.value.name),
                });
            }
            list(&binding.name, &parts)
        })
        .collect()
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
        PatKind::Word(value) => value.clone(),
        PatKind::String(value) => format!("{value:?}"),
        PatKind::Character(value) => format!("#{:?}", value.to_string()),
        PatKind::Boolean(value) => value.to_string(),
        PatKind::Unit => "()".into(),
        PatKind::Tuple(items) => list("tuple", &items.iter().map(pat_text).collect::<Vec<_>>()),
        PatKind::List(items) => list("list", &items.iter().map(pat_text).collect::<Vec<_>>()),
        PatKind::Record(fields, flexible) => {
            let mut parts: Vec<String> = fields
                .iter()
                .map(|(label, pattern)| format!("({label} {})", pat_text(pattern)))
                .collect();
            if *flexible {
                parts.push("...".into());
            }
            list("record", &parts)
        }
        PatKind::Constructor(name, argument) => format!("(con {name} {})", pat_text(argument)),
        PatKind::Cons(head, tail) => format!("(:: {} {})", pat_text(head), pat_text(tail)),
        PatKind::Layered(name, None, inner) => format!("(as {name} {})", pat_text(inner)),
        PatKind::Layered(name, Some(ty), inner) => {
            format!("(as {name} {} {})", ty_text(ty), pat_text(inner))
        }
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
