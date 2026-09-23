use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use crate::codegen::{Codegen, OptLevel};
use crate::error::CodegenError;
use crate::parser::{NumericValue, Parser, Program};

pub struct Repl {
    codegen: Codegen,
    module: cranelift_jit::JITModule,
    next_chunk: usize,
    variables: HashMap<String, NumericValue>,
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
        let codegen = Codegen::new(
            opt_level,
            debug_passes,
            false,
            dump_ir,
            dump_optimized_ir,
            verify,
            false,
            stats,
            false,
        );
        let module = codegen.new_jit_module()?;
        Ok(Self {
            codegen,
            module,
            next_chunk: 0,
            variables: HashMap::new(),
        })
    }

    fn parse(&self, source: &str, chunk: usize) -> miette::Result<Program> {
        let filename = format!("<repl:{chunk}>");
        let named_source = miette::NamedSource::new(filename.clone(), source.to_owned());
        Parser::from_repl_source_with_variables(source, &filename, self.variables.keys().cloned())
            .map_err(|error| miette::Report::new(error).with_source_code(named_source.clone()))?
            .parse()
            .map_err(|error| miette::Report::new(error).with_source_code(named_source))
    }

    fn execute(&mut self, source: &str) -> miette::Result<()> {
        let chunk = self.next_chunk;
        let program = self.parse(source, chunk)?;
        let name = format!("nassau_repl_{chunk}");
        let function = self
            .codegen
            .compile_jit_chunk(&mut self.module, &program, &name, &self.variables)
            .map_err(miette::Report::new)?;
        let function: extern "C" fn() -> i32 = unsafe { std::mem::transmute(function) };
        let result = function();
        for statement in &program.statements {
            if let crate::parser::StmtKind::Val(name, expr) = &statement.value {
                let value = evaluate_integer_expr(expr, &self.variables);
                self.variables.insert(name.clone(), value);
                match value {
                    NumericValue::Integer(value) => println!("val {name} = {value} : int"),
                    NumericValue::Real(value) => println!("val {name} = {value:?} : real"),
                }
            }
        }
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
            apply_numeric_op(lhs, rhs, variables, |a, b| a.wrapping_add(b), |a, b| a + b)
        }
        crate::parser::ExprKind::Subtract(lhs, rhs) => {
            apply_numeric_op(lhs, rhs, variables, |a, b| a.wrapping_sub(b), |a, b| a - b)
        }
        crate::parser::ExprKind::Multiply(lhs, rhs) => {
            apply_numeric_op(lhs, rhs, variables, |a, b| a.wrapping_mul(b), |a, b| a * b)
        }
        crate::parser::ExprKind::Divide(lhs, rhs) => {
            apply_numeric_op(lhs, rhs, variables, |_, _| unreachable!(), |a, b| a / b)
        }
        crate::parser::ExprKind::IntDivide(lhs, rhs) => {
            apply_numeric_op(lhs, rhs, variables, |a, b| a / b, |_, _| unreachable!())
        }
        _ => unreachable!(),
    }
}

fn apply_numeric_op(
    lhs: &crate::parser::Expr,
    rhs: &crate::parser::Expr,
    variables: &HashMap<String, NumericValue>,
    integer_op: impl FnOnce(i32, i32) -> i32,
    real_op: impl FnOnce(f64, f64) -> f64,
) -> NumericValue {
    match (
        evaluate_integer_expr(lhs, variables),
        evaluate_integer_expr(rhs, variables),
    ) {
        (NumericValue::Integer(lhs), NumericValue::Integer(rhs)) => {
            NumericValue::Integer(integer_op(lhs, rhs))
        }
        (NumericValue::Real(lhs), NumericValue::Real(rhs)) => NumericValue::Real(real_op(lhs, rhs)),
        _ => unreachable!(),
    }
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
    .map_err(miette::Report::new)?
    .run()
}
