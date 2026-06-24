use std::collections::HashMap;

use crate::error::ThisError;
use crate::parser::{Expr, ExprKind, Program, StmtKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Integer,
    Real,
    String,
    Boolean,
    Unit,
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
    Less,
    Equal,
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

    pub fn analyze_program<'a>(
        &mut self,
        program: &'a Program,
    ) -> Result<(), (SemanticError, &'a Expr)> {
        let mut scopes = self.scopes.clone();
        for statement in &program.statements {
            match &statement.value {
                StmtKind::Val(name, expr) => {
                    let ty = analyze_expr(expr, &scopes)?;
                    scopes.last_mut().unwrap().insert(name.clone(), ty);
                }
                StmtKind::Print(expr) => {
                    expect_type(analyze_expr(expr, &scopes)?, Type::String, expr)?;
                }
                StmtKind::Exit(expr) => {
                    expect_type(analyze_expr(expr, &scopes)?, Type::Unit, expr)?;
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
    match (operator, lhs) {
        (ComparisonOperator::Greater | ComparisonOperator::Less, Type::Integer | Type::Real)
        | (ComparisonOperator::Equal, Type::Integer | Type::Boolean) => Ok(Type::Boolean),
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
        ExprKind::String(_) => Ok(Type::String),
        ExprKind::Variable(name) => scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name))
            .copied()
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
        ExprKind::Greater(lhs, rhs) | ExprKind::Less(lhs, rhs) | ExprKind::Equal(lhs, rhs) => {
            let operator = match &expr.value {
                ExprKind::Greater(_, _) => ComparisonOperator::Greater,
                ExprKind::Less(_, _) => ComparisonOperator::Less,
                ExprKind::Equal(_, _) => ComparisonOperator::Equal,
                _ => unreachable!(),
            };
            let lhs = analyze_expr(lhs, scopes)?;
            let rhs = analyze_expr(rhs, scopes)?;
            comparison_result(operator, lhs, rhs)
                .map_err(|error| (SemanticError::InvalidComparison(error), expr))
        }
        ExprKind::Word8FromInt(expr) => {
            expect_type(analyze_expr(expr, scopes)?, Type::Integer, expr)?;
            Ok(Type::Integer)
        }
        ExprKind::PosixExit(expr) => {
            expect_type(analyze_expr(expr, scopes)?, Type::Integer, expr)?;
            Ok(Type::Unit)
        }
    }
}

fn expect_type(found: Type, expected: Type, expr: &Expr) -> Result<(), (SemanticError, &Expr)> {
    if found == expected {
        Ok(())
    } else {
        Err((SemanticError::TypeMismatch { expected, found }, expr))
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Integer => "int",
            Self::Real => "real",
            Self::String => "string",
            Self::Boolean => "bool",
            Self::Unit => "unit",
        })
    }
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Greater => ">",
            Self::Less => "<",
            Self::Equal => "=",
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
