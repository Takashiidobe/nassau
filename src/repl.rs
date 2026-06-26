use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use crate::codegen::{Codegen, CodegenOptions, OptLevel};
use crate::error::{CodegenError, SourceError};
use crate::parser::{NumericValue, Parser, Program};
use crate::sema::{self, Analyzer, ArithmeticOperator, ComparisonOperator, Type};

pub struct Repl {
    codegen: Codegen,
    module: cranelift_jit::JITModule,
    next_chunk: usize,
    variables: HashMap<String, NumericValue>,
    analyzer: Analyzer,
}

impl Repl {
    pub fn new(
        opt_level: OptLevel,
        debug_passes: bool,
        dump_ir: bool,
        dump_optimized_ir: bool,
        verify: bool,
        stats: bool,
    ) -> Result<Self, CodegenError> {
        let codegen = Codegen::new(CodegenOptions {
            opt_level,
            debug_passes,
            asm: false,
            dump_ir,
            dump_optimized_ir,
            verify,
            timings: false,
            stats,
            objdump: false,
        });
        let module = codegen.new_jit_module()?;
        Ok(Self {
            codegen,
            module,
            next_chunk: 0,
            variables: HashMap::new(),
            analyzer: Analyzer::new(),
        })
    }

    fn parse(&self, source: &str, chunk: usize) -> miette::Result<Program> {
        let filename = format!("<repl:{chunk}>");
        let named_source = miette::NamedSource::new(filename.clone(), source.to_owned());
        Parser::from_repl_source(source, &filename)
            .map_err(|error| {
                miette::Report::new(SourceError::from_span(error))
                    .with_source_code(named_source.clone())
            })?
            .parse()
            .map_err(|error| {
                miette::Report::new(SourceError::from_span(error)).with_source_code(named_source)
            })
    }

    fn execute(&mut self, source: &str) -> miette::Result<()> {
        let chunk = self.next_chunk;
        let program = self.parse(source, chunk)?;
        let mut analyzer = self.analyzer.clone();
        let named_source = miette::NamedSource::new(format!("<repl:{chunk}>"), source.to_owned());
        analyzer
            .analyze_program(&program)
            .map_err(|(error, expr)| {
                miette::Report::new(SourceError::new(error, expr.source_span()))
                    .with_source_code(named_source)
            })?;
        let name = format!("nassau_repl_{chunk}");
        let function = self
            .codegen
            .compile_jit_chunk(&mut self.module, &program, &name, &self.variables)
            .map_err(miette::Report::msg)?;
        let function: extern "C" fn() -> i32 = unsafe { std::mem::transmute(function) };
        let result = function();
        for statement in &program.statements {
            if let crate::parser::StmtKind::Val(name, expr) = &statement.value {
                let value = evaluate_integer_expr(expr, &self.variables);
                self.variables.insert(name.clone(), value);
                match value {
                    NumericValue::Integer(value) => println!("val {name} = {value} : int"),
                    NumericValue::Real(value) => println!("val {name} = {value} : real"),
                    NumericValue::Boolean(value) => println!("val {name} = {value} : bool"),
                }
            }
        }
        self.analyzer = analyzer;
        if program.statements.is_empty() {
            println!("val it = {result} : int");
        }
        self.next_chunk += 1;
        Ok(())
    }

    pub fn run(&mut self) -> miette::Result<()> {
        let stdin = io::stdin();
        let mut input = stdin.lock();
        let mut line = String::new();
        loop {
            print!("nassau> ");
            io::stdout()
                .flush()
                .map_err(|error| miette::miette!("{error}"))?;
            line.clear();
            let bytes = input
                .read_line(&mut line)
                .map_err(|error| miette::miette!("{error}"))?;
            if bytes == 0 {
                break;
            }
            if line.trim().is_empty() {
                continue;
            }
            if let Err(error) = self.execute(&line) {
                eprintln!("{error:?}");
            }
        }
        Ok(())
    }
}

fn evaluate_integer_expr(
    expr: &crate::parser::Expr,
    variables: &HashMap<String, NumericValue>,
) -> NumericValue {
    match &expr.value {
        crate::parser::ExprKind::Integer(value) => NumericValue::Integer(*value as i32),
        crate::parser::ExprKind::Real(value) => NumericValue::Real(*value),
        crate::parser::ExprKind::Variable(name) => variables[name],
        crate::parser::ExprKind::Add(lhs, rhs) => {
            apply_numeric_op(ArithmeticOperator::Add, lhs, rhs, variables)
        }
        crate::parser::ExprKind::Subtract(lhs, rhs) => {
            apply_numeric_op(ArithmeticOperator::Subtract, lhs, rhs, variables)
        }
        crate::parser::ExprKind::Multiply(lhs, rhs) => {
            apply_numeric_op(ArithmeticOperator::Multiply, lhs, rhs, variables)
        }
        crate::parser::ExprKind::Divide(lhs, rhs) => {
            apply_numeric_op(ArithmeticOperator::Divide, lhs, rhs, variables)
        }
        crate::parser::ExprKind::IntDivide(lhs, rhs) => {
            apply_numeric_op(ArithmeticOperator::IntDivide, lhs, rhs, variables)
        }
        crate::parser::ExprKind::Greater(lhs, rhs) => {
            apply_comparison(ComparisonOperator::Greater, lhs, rhs, variables)
        }
        crate::parser::ExprKind::GreaterEqual(lhs, rhs) => {
            apply_comparison(ComparisonOperator::GreaterEqual, lhs, rhs, variables)
        }
        crate::parser::ExprKind::Less(lhs, rhs) => {
            apply_comparison(ComparisonOperator::Less, lhs, rhs, variables)
        }
        crate::parser::ExprKind::LessEqual(lhs, rhs) => {
            apply_comparison(ComparisonOperator::LessEqual, lhs, rhs, variables)
        }
        crate::parser::ExprKind::Equal(lhs, rhs) => {
            apply_comparison(ComparisonOperator::Equal, lhs, rhs, variables)
        }
        crate::parser::ExprKind::NotEqual(lhs, rhs) => {
            apply_comparison(ComparisonOperator::NotEqual, lhs, rhs, variables)
        }
        _ => unreachable!(),
    }
}

fn apply_numeric_op(
    operator: ArithmeticOperator,
    lhs: &crate::parser::Expr,
    rhs: &crate::parser::Expr,
    variables: &HashMap<String, NumericValue>,
) -> NumericValue {
    let lhs = evaluate_integer_expr(lhs, variables);
    let rhs = evaluate_integer_expr(rhs, variables);
    let result_type = sema::arithmetic_result(operator, numeric_type(lhs), numeric_type(rhs))
        .expect("semantic analysis has already validated arithmetic");
    match (operator, result_type, lhs, rhs) {
        (
            ArithmeticOperator::Add,
            Type::Integer,
            NumericValue::Integer(lhs),
            NumericValue::Integer(rhs),
        ) => NumericValue::Integer(lhs.wrapping_add(rhs)),
        (
            ArithmeticOperator::Subtract,
            Type::Integer,
            NumericValue::Integer(lhs),
            NumericValue::Integer(rhs),
        ) => NumericValue::Integer(lhs.wrapping_sub(rhs)),
        (
            ArithmeticOperator::Multiply,
            Type::Integer,
            NumericValue::Integer(lhs),
            NumericValue::Integer(rhs),
        ) => NumericValue::Integer(lhs.wrapping_mul(rhs)),
        (
            ArithmeticOperator::IntDivide,
            Type::Integer,
            NumericValue::Integer(lhs),
            NumericValue::Integer(rhs),
        ) => NumericValue::Integer(lhs / rhs),
        (ArithmeticOperator::Add, Type::Real, NumericValue::Real(lhs), NumericValue::Real(rhs)) => {
            NumericValue::Real(lhs + rhs)
        }
        (
            ArithmeticOperator::Subtract,
            Type::Real,
            NumericValue::Real(lhs),
            NumericValue::Real(rhs),
        ) => NumericValue::Real(lhs - rhs),
        (
            ArithmeticOperator::Multiply,
            Type::Real,
            NumericValue::Real(lhs),
            NumericValue::Real(rhs),
        ) => NumericValue::Real(lhs * rhs),
        (
            ArithmeticOperator::Divide,
            Type::Real,
            NumericValue::Real(lhs),
            NumericValue::Real(rhs),
        ) => NumericValue::Real(lhs / rhs),
        _ => unreachable!(),
    }
}

fn numeric_type(value: NumericValue) -> Type {
    match value {
        NumericValue::Integer(_) => Type::Integer,
        NumericValue::Real(_) => Type::Real,
        NumericValue::Boolean(_) => Type::Boolean,
    }
}

fn apply_comparison(
    operator: ComparisonOperator,
    lhs: &crate::parser::Expr,
    rhs: &crate::parser::Expr,
    variables: &HashMap<String, NumericValue>,
) -> NumericValue {
    let lhs = evaluate_integer_expr(lhs, variables);
    let rhs = evaluate_integer_expr(rhs, variables);
    sema::comparison_result(operator, numeric_type(lhs), numeric_type(rhs))
        .expect("semantic analysis has already validated comparisons");
    let result = match (operator, lhs, rhs) {
        (ComparisonOperator::Greater, NumericValue::Integer(lhs), NumericValue::Integer(rhs)) => {
            lhs > rhs
        }
        (
            ComparisonOperator::GreaterEqual,
            NumericValue::Integer(lhs),
            NumericValue::Integer(rhs),
        ) => lhs >= rhs,
        (ComparisonOperator::Less, NumericValue::Integer(lhs), NumericValue::Integer(rhs)) => {
            lhs < rhs
        }
        (ComparisonOperator::LessEqual, NumericValue::Integer(lhs), NumericValue::Integer(rhs)) => {
            lhs <= rhs
        }
        (ComparisonOperator::Equal, NumericValue::Integer(lhs), NumericValue::Integer(rhs)) => {
            lhs == rhs
        }
        (ComparisonOperator::NotEqual, NumericValue::Integer(lhs), NumericValue::Integer(rhs)) => {
            lhs != rhs
        }
        (ComparisonOperator::Greater, NumericValue::Real(lhs), NumericValue::Real(rhs)) => {
            lhs > rhs
        }
        (ComparisonOperator::GreaterEqual, NumericValue::Real(lhs), NumericValue::Real(rhs)) => {
            lhs >= rhs
        }
        (ComparisonOperator::Less, NumericValue::Real(lhs), NumericValue::Real(rhs)) => lhs < rhs,
        (ComparisonOperator::LessEqual, NumericValue::Real(lhs), NumericValue::Real(rhs)) => {
            lhs <= rhs
        }
        (ComparisonOperator::Equal, NumericValue::Boolean(lhs), NumericValue::Boolean(rhs)) => {
            lhs == rhs
        }
        (ComparisonOperator::NotEqual, NumericValue::Boolean(lhs), NumericValue::Boolean(rhs)) => {
            lhs != rhs
        }
        _ => unreachable!(),
    };
    NumericValue::Boolean(result)
}

pub fn run(
    opt_level: OptLevel,
    debug_passes: bool,
    dump_ir: bool,
    dump_optimized_ir: bool,
    verify: bool,
    stats: bool,
) -> miette::Result<()> {
    Repl::new(
        opt_level,
        debug_passes,
        dump_ir,
        dump_optimized_ir,
        verify,
        stats,
    )
    .map_err(miette::Report::msg)?
    .run()
}
