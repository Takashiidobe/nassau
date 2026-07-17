use std::collections::HashMap;

use crate::error::ThisError;
use crate::parser::{DeclKind, Expr, ExprKind, Program, StmtKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Integer,
    Real,
    String,
    Boolean,
    Unit,
    List(Box<Type>),
    Variable(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    IntDivide,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, ThisError)]
pub enum SemanticError {
    #[error("unbound variable '{0}'")]
    UnboundVariable(String),
    #[error("expected {expected}, found {found}")]
    TypeMismatch { expected: Type, found: Type },
    #[error(transparent)]
    InvalidArithmetic(#[from] ArithmeticTypeError),
    #[error(transparent)]
    InvalidComparison(#[from] ComparisonTypeError),
    #[error("integer literal does not fit in i32")]
    IntegerOutOfRange,
    #[error("{0} is not supported yet")]
    Unsupported(&'static str),
}

#[derive(Debug, ThisError)]
pub enum ArithmeticTypeError {
    #[error("'/' expects real operands, found {lhs} and {rhs}")]
    RequiredReal { lhs: Type, rhs: Type },
    #[error("'div' expects integer operands, found {lhs} and {rhs}")]
    RequiredInteger { lhs: Type, rhs: Type },
    #[error("arithmetic operands must have the same numeric type, found {lhs} and {rhs}")]
    MismatchedTypes { lhs: Type, rhs: Type },
}

#[derive(Debug, ThisError)]
pub enum ComparisonTypeError {
    #[error("comparison operands must have the same type, found {lhs} and {rhs}")]
    MismatchedTypes { lhs: Type, rhs: Type },
    #[error("operator '{operator}' does not support {ty}")]
    UnsupportedOperands {
        operator: ComparisonOperator,
        ty: Type,
    },
}

#[derive(Clone, Debug)]
pub struct Analyzer {
    scopes: Vec<HashMap<String, Type>>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    #[expect(
        dead_code,
        reason = "Lexical constructs will use explicit nested scopes."
    )]
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    #[expect(
        dead_code,
        reason = "Lexical constructs will use explicit nested scopes."
    )]
    pub fn pop_scope(&mut self) -> bool {
        if self.scopes.len() == 1 {
            false
        } else {
            self.scopes.pop();
            true
        }
    }

    pub fn analyze_program(
        &mut self,
        program: &Program,
    ) -> Result<(), (SemanticError, miette::SourceSpan)> {
        let span_of = |(error, expr): (SemanticError, &Expr)| (error, expr.source_span());
        let mut scopes = self.scopes.clone();
        for statement in &program.statements {
            match &statement.value {
                StmtKind::Val(name, expr) => {
                    let ty = analyze_expr(expr, &scopes).map_err(span_of)?;
                    scopes.last_mut().unwrap().insert(name.clone(), ty);
                }
                StmtKind::Print(expr) => {
                    expect_type(
                        analyze_expr(expr, &scopes).map_err(span_of)?,
                        Type::String,
                        expr,
                    )
                    .map_err(span_of)?;
                }
                StmtKind::Exit(expr) => {
                    expect_type(
                        analyze_expr(expr, &scopes).map_err(span_of)?,
                        Type::Unit,
                        expr,
                    )
                    .map_err(span_of)?;
                }
                StmtKind::Declaration(declaration) => {
                    let name = match &declaration.value {
                        DeclKind::Val {
                            recursive: true, ..
                        } => "val rec declarations",
                        DeclKind::Val { .. } => "val declarations with patterns",
                        DeclKind::Fun(_) => "fun declarations",
                        DeclKind::Type(_) => "type declarations",
                        DeclKind::Datatype { .. } | DeclKind::DatatypeCopy { .. } => {
                            "datatype declarations"
                        }
                        DeclKind::Abstype { .. } => "abstype declarations",
                        DeclKind::Exception(_) => "exception declarations",
                        DeclKind::Local(..) => "local declarations",
                        DeclKind::Fixity { .. } => "fixity declarations",
                    };
                    return Err((SemanticError::Unsupported(name), declaration.source_span()));
                }
            }
        }
        self.scopes = scopes;
        Ok(())
    }
}

pub fn arithmetic_result(
    operator: ArithmeticOperator,
    lhs: Type,
    rhs: Type,
) -> Result<Type, ArithmeticTypeError> {
    match (operator, lhs, rhs) {
        (ArithmeticOperator::Add, Type::Integer, Type::Integer)
        | (ArithmeticOperator::Subtract, Type::Integer, Type::Integer)
        | (ArithmeticOperator::Multiply, Type::Integer, Type::Integer)
        | (ArithmeticOperator::IntDivide, Type::Integer, Type::Integer) => Ok(Type::Integer),
        (ArithmeticOperator::Add, Type::Real, Type::Real)
        | (ArithmeticOperator::Subtract, Type::Real, Type::Real)
        | (ArithmeticOperator::Multiply, Type::Real, Type::Real)
        | (ArithmeticOperator::Divide, Type::Real, Type::Real) => Ok(Type::Real),
        (ArithmeticOperator::Divide, lhs, rhs) => {
            Err(ArithmeticTypeError::RequiredReal { lhs, rhs })
        }
        (ArithmeticOperator::IntDivide, lhs, rhs) => {
            Err(ArithmeticTypeError::RequiredInteger { lhs, rhs })
        }
        (_, lhs, rhs) => Err(ArithmeticTypeError::MismatchedTypes { lhs, rhs }),
    }
}

pub fn comparison_result(
    operator: ComparisonOperator,
    lhs: Type,
    rhs: Type,
) -> Result<Type, ComparisonTypeError> {
    if lhs != rhs {
        return Err(ComparisonTypeError::MismatchedTypes { lhs, rhs });
    }
    match (operator, lhs.clone()) {
        (
            ComparisonOperator::Greater
            | ComparisonOperator::GreaterEqual
            | ComparisonOperator::Less
            | ComparisonOperator::LessEqual,
            Type::Integer | Type::Real,
        )
        | (
            ComparisonOperator::Equal | ComparisonOperator::NotEqual,
            Type::Integer | Type::Boolean,
        ) => Ok(Type::Boolean),
        _ => Err(ComparisonTypeError::UnsupportedOperands { operator, ty: lhs }),
    }
}

fn analyze_expr<'a>(
    expr: &'a Expr,
    scopes: &[HashMap<String, Type>],
) -> Result<Type, (SemanticError, &'a Expr)> {
    match &expr.value {
        ExprKind::Integer(value) => i32::try_from(*value)
            .map(|_| Type::Integer)
            .map_err(|_| (SemanticError::IntegerOutOfRange, expr)),
        ExprKind::Real(_) => Ok(Type::Real),
        ExprKind::Boolean(_) => Ok(Type::Boolean),
        ExprKind::String(_) => Ok(Type::String),
        ExprKind::List(elements) => {
            let mut element_type = Type::Variable(expr.start.offset);
            for element in elements {
                let found = analyze_expr(element, scopes)?;
                element_type = unify(element_type, found).map_err(|(expected, found)| {
                    (SemanticError::TypeMismatch { expected, found }, element)
                })?;
            }
            Ok(Type::List(Box::new(element_type)))
        }
        ExprKind::Variable(name) => scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .cloned()
            .ok_or_else(|| (SemanticError::UnboundVariable(name.clone()), expr)),
        ExprKind::Add(lhs, rhs)
        | ExprKind::Subtract(lhs, rhs)
        | ExprKind::Multiply(lhs, rhs)
        | ExprKind::Divide(lhs, rhs)
        | ExprKind::IntDivide(lhs, rhs) => {
            let operator = match &expr.value {
                ExprKind::Add(_, _) => ArithmeticOperator::Add,
                ExprKind::Subtract(_, _) => ArithmeticOperator::Subtract,
                ExprKind::Multiply(_, _) => ArithmeticOperator::Multiply,
                ExprKind::Divide(_, _) => ArithmeticOperator::Divide,
                ExprKind::IntDivide(_, _) => ArithmeticOperator::IntDivide,
                _ => unreachable!(),
            };
            let lhs = analyze_expr(lhs, scopes)?;
            let rhs = analyze_expr(rhs, scopes)?;
            arithmetic_result(operator, lhs, rhs)
                .map_err(|error| (SemanticError::InvalidArithmetic(error), expr))
        }
        ExprKind::Greater(lhs, rhs)
        | ExprKind::GreaterEqual(lhs, rhs)
        | ExprKind::Less(lhs, rhs)
        | ExprKind::LessEqual(lhs, rhs)
        | ExprKind::Equal(lhs, rhs)
        | ExprKind::NotEqual(lhs, rhs) => {
            let operator = match &expr.value {
                ExprKind::Greater(_, _) => ComparisonOperator::Greater,
                ExprKind::GreaterEqual(_, _) => ComparisonOperator::GreaterEqual,
                ExprKind::Less(_, _) => ComparisonOperator::Less,
                ExprKind::LessEqual(_, _) => ComparisonOperator::LessEqual,
                ExprKind::Equal(_, _) => ComparisonOperator::Equal,
                ExprKind::NotEqual(_, _) => ComparisonOperator::NotEqual,
                _ => unreachable!(),
            };
            let lhs = analyze_expr(lhs, scopes)?;
            let rhs = analyze_expr(rhs, scopes)?;
            comparison_result(operator, lhs, rhs)
                .map_err(|error| (SemanticError::InvalidComparison(error), expr))
        }
        ExprKind::If(condition, consequent, alternative) => {
            expect_type(analyze_expr(condition, scopes)?, Type::Boolean, condition)?;
            let consequent_type = analyze_expr(consequent, scopes)?;
            let alternative_type = analyze_expr(alternative, scopes)?;
            unify(consequent_type, alternative_type).map_err(|(expected, found)| {
                (SemanticError::TypeMismatch { expected, found }, expr)
            })
        }
        ExprKind::Word8FromInt(expr) => {
            expect_type(analyze_expr(expr, scopes)?, Type::Integer, expr)?;
            Ok(Type::Integer)
        }
        ExprKind::PosixExit(expr) => {
            expect_type(analyze_expr(expr, scopes)?, Type::Integer, expr)?;
            Ok(Type::Unit)
        }
        // Parsed in full, but the type checker and backend do not handle these yet.
        other => Err((SemanticError::Unsupported(unsupported_name(other)), expr)),
    }
}

fn unsupported_name(kind: &ExprKind) -> &'static str {
    match kind {
        ExprKind::Character(_) => "character literals",
        ExprKind::Word(_) => "word literals",
        ExprKind::Unit => "the unit value",
        ExprKind::Tuple(_) => "tuples",
        ExprKind::Record(_) => "records",
        ExprKind::Selector(_) => "record selectors",
        ExprKind::Apply(..) => "function application",
        ExprKind::Infix(..) => "this infix operator",
        ExprKind::AndAlso(..) => "andalso",
        ExprKind::OrElse(..) => "orelse",
        ExprKind::Sequence(_) => "expression sequences",
        ExprKind::Let(..) => "let expressions",
        ExprKind::Case(..) => "case expressions",
        ExprKind::Fn(_) => "fn expressions",
        ExprKind::While(..) => "while loops",
        ExprKind::Raise(_) => "raise",
        ExprKind::Handle(..) => "handle",
        ExprKind::Typed(..) => "type annotations",
        _ => "this expression",
    }
}

fn expect_type(found: Type, expected: Type, expr: &Expr) -> Result<(), (SemanticError, &Expr)> {
    if unify(found.clone(), expected.clone()).is_ok() {
        Ok(())
    } else {
        Err((SemanticError::TypeMismatch { expected, found }, expr))
    }
}

fn unify(lhs: Type, rhs: Type) -> Result<Type, (Type, Type)> {
    match (lhs, rhs) {
        (Type::Variable(_), ty) | (ty, Type::Variable(_)) => Ok(ty),
        (Type::List(a), Type::List(b)) => unify(*a, *b).map(|ty| Type::List(Box::new(ty))),
        (a, b) if a == b => Ok(a),
        (a, b) => Err((a, b)),
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => formatter.write_str("int"),
            Self::Real => formatter.write_str("real"),
            Self::String => formatter.write_str("string"),
            Self::Boolean => formatter.write_str("bool"),
            Self::Unit => formatter.write_str("unit"),
            Self::List(e) => write!(formatter, "{e} list"),
            Self::Variable(id) => write!(formatter, "'a{id}"),
        }
    }
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Equal => "=",
            Self::NotEqual => "<>",
        })
    }
}

impl std::fmt::Display for ArithmeticOperator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::IntDivide => "div",
        })
    }
}
