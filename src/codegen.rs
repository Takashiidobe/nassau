use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use clap::ValueEnum;
use cranelift_codegen::ir::{
    AbiParam, InstBuilder,
    condcodes::{FloatCC, IntCC},
    immediates::Ieee64,
    types,
};
use cranelift_codegen::settings::Configurable;
use cranelift_codegen::{self, settings};
use cranelift_control::ControlPlane;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, FuncId, Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::error::CodegenError;
use crate::parser::{ExprKind, NumericValue, Program, StmtKind};
use crate::sema::{self, ArithmeticOperator, ComparisonOperator, Type};
use crate::value;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OptLevel {
    None,
    Speed,
    #[value(name = "speed-and-size")]
    SpeedAndSize,
}

impl OptLevel {
    fn as_cranelift(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Speed => "speed",
            Self::SpeedAndSize => "speed_and_size",
        }
    }
}

pub struct Codegen {
    opt_level: OptLevel,
    debug_passes: bool,
    asm: bool,
    dump_ir: bool,
    dump_optimized_ir: bool,
    verify: bool,
    timings: bool,
    stats: bool,
    objdump: bool,
}

pub struct CodegenOptions {
    pub opt_level: OptLevel,
    pub debug_passes: bool,
    pub asm: bool,
    pub dump_ir: bool,
    pub dump_optimized_ir: bool,
    pub verify: bool,
    pub timings: bool,
    pub stats: bool,
    pub objdump: bool,
}

struct ExprEnvironment<'a> {
    variables: &'a HashMap<String, cranelift_codegen::ir::Value>,
    allocator: cranelift_codegen::ir::FuncRef,
    pointer_type: cranelift_codegen::ir::Type,
}

struct FunctionBuild {
    id: FuncId,
    context: cranelift_codegen::Context,
    frontend: Duration,
    optimization: Duration,
    codegen: Duration,
}

impl Codegen {
    pub fn new(options: CodegenOptions) -> Self {
        Self {
            opt_level: options.opt_level,
            debug_passes: options.debug_passes,
            asm: options.asm,
            dump_ir: options.dump_ir,
            dump_optimized_ir: options.dump_optimized_ir,
            verify: options.verify,
            timings: options.timings,
            stats: options.stats,
            objdump: options.objdump,
        }
    }

    fn isa(&self, jit: bool) -> Result<cranelift_codegen::isa::OwnedTargetIsa, CodegenError> {
        let mut flag_builder = settings::builder();
        flag_builder
            .set("opt_level", self.opt_level.as_cranelift())
            .map_err(|error| CodegenError::Backend(error.to_string()))?;
        if jit {
            flag_builder
                .set("use_colocated_libcalls", "false")
                .map_err(|error| CodegenError::Backend(error.to_string()))?;
            if cfg!(target_arch = "x86_64") {
                flag_builder
                    .set("is_pic", "true")
                    .map_err(|error| CodegenError::Backend(error.to_string()))?;
            }
        }
        cranelift_native::builder()
            .map_err(|error| CodegenError::Backend(error.to_string()))?
            .finish(settings::Flags::new(flag_builder))
            .map_err(|error| CodegenError::Backend(error.to_string()))
    }

    pub fn new_jit_module(&self) -> Result<JITModule, CodegenError> {
        let builder = JITBuilder::with_isa(self.isa(true)?, default_libcall_names());
        Ok(JITModule::new(builder))
    }

    pub fn compile_jit_chunk(
        &self,
        module: &mut JITModule,
        program: &Program,
        name: &str,
        initial_variables: &HashMap<String, NumericValue>,
    ) -> Result<*const u8, CodegenError> {
        let function = self.define_program(module, program, name, initial_variables)?;
        module
            .finalize_definitions()
            .map_err(|error| CodegenError::Backend(error.to_string()))?;
        Ok(module.get_finalized_function(function.id))
    }

    pub fn compile(&self, program: &Program, output: &Path) -> Result<(), CodegenError> {
        let total_start = Instant::now();
        let isa = self.isa(false)?;
        let object_builder = ObjectBuilder::new(isa, "nassau", default_libcall_names())
            .map_err(|error| CodegenError::Backend(error.to_string()))?;
        let mut module = ObjectModule::new(object_builder);
        let function = self.define_program(&mut module, program, "main", &HashMap::new())?;

        if self.debug_passes || self.asm {
            let disassembly = function
                .context
                .compiled_code()
                .and_then(|compiled| compiled.vcode.as_deref())
                .ok_or_else(|| {
                    CodegenError::Message(
                        "target does not provide a textual assembly listing".into(),
                    )
                })?;
            if self.debug_passes {
                println!("== Cranelift machine instructions ==\n{disassembly}");
            }
            if self.asm {
                fs::write(output, disassembly)
                    .map_err(|error| CodegenError::Io(error.to_string()))?;
                self.print_timings(&function, Duration::ZERO, total_start.elapsed());
                return Ok(());
            }
        }

        let object = module
            .finish()
            .emit()
            .map_err(|error| CodegenError::Backend(error.to_string()))?;
        let object_path = env::temp_dir().join(format!("nassau-{}.o", std::process::id()));
        fs::write(&object_path, object).map_err(|error| CodegenError::Io(error.to_string()))?;
        if self.objdump {
            let objdump = Command::new("objdump")
                .args(["-drwC"])
                .arg(&object_path)
                .output()
                .map_err(|error| {
                    CodegenError::Tool(format!("failed to invoke objdump: {error}"))
                })?;
            if !objdump.status.success() {
                let _ = fs::remove_file(&object_path);
                return Err(CodegenError::Tool(
                    String::from_utf8_lossy(&objdump.stderr).trim().to_string(),
                ));
            }
            println!(
                "== Object disassembly ==\n{}",
                String::from_utf8_lossy(&objdump.stdout)
            );
        }
        let linker = env::var("NASSAU_CC").unwrap_or_else(|_| "cc".to_string());
        let link_start = Instant::now();
        let link_result = Command::new(&linker)
            .args(["-o"])
            .arg(output)
            .arg(&object_path)
            .output()
            .map_err(|error| CodegenError::Tool(format!("failed to invoke {linker}: {error}")))?;
        let _ = fs::remove_file(&object_path);
        if !link_result.status.success() {
            return Err(CodegenError::Linker(
                String::from_utf8_lossy(&link_result.stderr)
                    .trim()
                    .to_string(),
            ));
        }
        self.print_timings(&function, link_start.elapsed(), total_start.elapsed());
        Ok(())
    }

    fn define_program<M: Module>(
        &self,
        module: &mut M,
        program: &Program,
        name: &str,
        initial_variables: &HashMap<String, NumericValue>,
    ) -> Result<FunctionBuild, CodegenError> {
        let total_start = Instant::now();
        let frontend_config = module.isa().frontend_config();
        let mut signature = module.make_signature();
        signature.returns.push(AbiParam::new(types::I32));
        let function = module
            .declare_function(name, Linkage::Export, &signature)
            .map_err(|error| CodegenError::Backend(error.to_string()))?;
        let mut context = module.make_context();
        context.func.signature = signature;
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let block = builder.create_block();
        builder.switch_to_block(block);
        builder.seal_block(block);

        let pointer_type = module.isa().pointer_type();
        let has_print = program
            .statements
            .iter()
            .any(|statement| matches!(statement.value, StmtKind::Print(_)));
        let has_exit = program
            .statements
            .iter()
            .any(|statement| matches!(statement.value, StmtKind::Exit(_)));
        let printf = if has_print {
            let mut signature = module.make_signature();
            signature.params.push(AbiParam::new(pointer_type));
            signature.params.push(AbiParam::new(pointer_type));
            signature.returns.push(AbiParam::new(types::I32));
            Some(
                module
                    .declare_function("printf", Linkage::Import, &signature)
                    .map_err(|error| CodegenError::Backend(error.to_string()))?,
            )
        } else {
            None
        };
        let fflush = if has_print {
            let mut signature = module.make_signature();
            signature.params.push(AbiParam::new(pointer_type));
            signature.returns.push(AbiParam::new(types::I32));
            Some(
                module
                    .declare_function("fflush", Linkage::Import, &signature)
                    .map_err(|error| CodegenError::Backend(error.to_string()))?,
            )
        } else {
            None
        };
        let exit = if has_exit {
            let mut signature = module.make_signature();
            signature.params.push(AbiParam::new(types::I32));
            Some(
                module
                    .declare_function("exit", Linkage::Import, &signature)
                    .map_err(|error| CodegenError::Backend(error.to_string()))?,
            )
        } else {
            None
        };
        let mut allocator_signature = module.make_signature();
        allocator_signature.params.push(AbiParam::new(pointer_type));
        allocator_signature
            .returns
            .push(AbiParam::new(pointer_type));
        let allocator = module
            .declare_function("malloc", Linkage::Import, &allocator_signature)
            .map_err(|error| CodegenError::Backend(error.to_string()))?;
        let format_data = if has_print {
            let id = module
                .declare_data(&format!("{name}_format"), Linkage::Local, false, false)
                .map_err(|error| CodegenError::Backend(error.to_string()))?;
            let mut data = DataDescription::new();
            data.define(b"%s\0".to_vec().into_boxed_slice());
            module
                .define_data(id, &data)
                .map_err(|error| CodegenError::Backend(error.to_string()))?;
            Some(id)
        } else {
            None
        };
        let mut string_data = Vec::new();
        for (index, statement) in program.statements.iter().enumerate() {
            if let StmtKind::Print(expr) = &statement.value
                && let ExprKind::String(value) = &expr.value
            {
                let id = module
                    .declare_data(
                        &format!("{name}_string_{index}"),
                        Linkage::Local,
                        false,
                        false,
                    )
                    .map_err(|error| CodegenError::Backend(error.to_string()))?;
                let mut data = DataDescription::new();
                let mut bytes = value.as_bytes().to_vec();
                bytes.push(0);
                data.define(bytes.into_boxed_slice());
                module
                    .define_data(id, &data)
                    .map_err(|error| CodegenError::Backend(error.to_string()))?;
                string_data.push((index, id));
            }
        }
        let mut variables = initial_variables
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    match value {
                        NumericValue::Integer(value) => {
                            builder.ins().iconst(types::I32, i64::from(*value))
                        }
                        NumericValue::Real(value) => {
                            builder.ins().f64const(Ieee64::with_float(*value))
                        }
                        NumericValue::Boolean(value) => {
                            builder.ins().iconst(types::I8, i64::from(*value))
                        }
                        NumericValue::List(value) => {
                            builder.ins().iconst(pointer_type, *value as i64)
                        }
                    },
                )
            })
            .collect::<HashMap<_, _>>();
        for (index, statement) in program.statements.iter().enumerate() {
            match &statement.value {
                StmtKind::Val(name, expr) => {
                    let allocator = module.declare_func_in_func(allocator, builder.func);
                    let environment = ExprEnvironment {
                        variables: &variables,
                        allocator,
                        pointer_type,
                    };
                    let value = Self::compile_expr(expr, &mut builder, &environment)?;
                    variables.insert(name.clone(), value);
                }
                StmtKind::Print(_) => {
                    let format_id = format_data.unwrap();
                    let string_id = string_data.iter().find(|(i, _)| *i == index).unwrap().1;
                    let format_global = module.declare_data_in_func(format_id, builder.func);
                    let string_global = module.declare_data_in_func(string_id, builder.func);
                    let format_value = builder.ins().symbol_value(pointer_type, format_global);
                    let string_value = builder.ins().symbol_value(pointer_type, string_global);
                    let function = module.declare_func_in_func(printf.unwrap(), builder.func);
                    builder.ins().call(function, &[format_value, string_value]);
                    let function = module.declare_func_in_func(fflush.unwrap(), builder.func);
                    let null = builder.ins().iconst(pointer_type, 0);
                    builder.ins().call(function, &[null]);
                }
                StmtKind::Declaration(_) => {
                    unreachable!("semantic analysis rejects declarations the backend lacks")
                }
                StmtKind::Exit(expr) => {
                    let ExprKind::PosixExit(word8) = &expr.value else {
                        unreachable!()
                    };
                    let ExprKind::Word8FromInt(integer) = &word8.value else {
                        unreachable!()
                    };
                    let allocator = module.declare_func_in_func(allocator, builder.func);
                    let environment = ExprEnvironment {
                        variables: &variables,
                        allocator,
                        pointer_type,
                    };
                    let value = Self::compile_expr(integer, &mut builder, &environment)?;
                    if builder.func.dfg.value_type(value) != types::I32 {
                        return Err(CodegenError::Message(
                            "Posix.Process.exit expects an integer".into(),
                        ));
                    }
                    let function = module.declare_func_in_func(exit.unwrap(), builder.func);
                    let code = builder.ins().urem_imm_u(value, 256);
                    builder.ins().call(function, &[code]);
                }
            }
        }
        let result = builder.ins().iconst(types::I32, i64::from(program.result));
        builder.ins().return_(&[result]);
        builder.finalize(frontend_config);
        let frontend = total_start.elapsed();
        let dump_ir = self.debug_passes || self.dump_ir;
        let dump_optimized_ir = self.debug_passes || self.dump_optimized_ir;
        let needs_manual_optimization = dump_optimized_ir || self.verify || self.stats;
        if self.verify {
            cranelift_codegen::verify_function(&context.func, module.isa())
                .map_err(|error| CodegenError::Backend(error.to_string()))?;
        }
        if dump_ir {
            println!(
                "== Cranelift IR before optimization ==\n{}",
                context.func.display()
            );
        }
        let optimization_start = Instant::now();
        if needs_manual_optimization {
            context
                .optimize(module.isa(), &mut ControlPlane::default())
                .map_err(|error| CodegenError::Backend(error.to_string()))?;
        }
        let optimization = optimization_start.elapsed();
        if self.verify && needs_manual_optimization {
            cranelift_codegen::verify_function(&context.func, module.isa())
                .map_err(|error| CodegenError::Backend(error.to_string()))?;
        }
        if dump_optimized_ir {
            println!(
                "== Cranelift IR after optimization ==\n{}",
                context.func.display()
            );
        }
        context.set_disasm(self.debug_passes || self.asm);
        let codegen_start = Instant::now();
        module
            .define_function(function, &mut context)
            .map_err(|error| CodegenError::Backend(error.to_string()))?;
        let codegen = codegen_start.elapsed();
        if self.stats {
            let blocks = context.func.layout.blocks().count();
            let instructions = context
                .func
                .layout
                .blocks()
                .map(|block| context.func.layout.block_insts(block).count())
                .sum::<usize>();
            let code_size = context
                .compiled_code()
                .map(|compiled| compiled.code_info().total_size)
                .unwrap_or_default();
            println!(
                "== Cranelift stats ==\nblocks: {blocks}\nIR instructions: {instructions}\ncode bytes: {code_size}"
            );
        }
        Ok(FunctionBuild {
            id: function,
            context,
            frontend,
            optimization,
            codegen,
        })
    }

    fn compile_expr(
        expr: &crate::parser::Expr,
        builder: &mut FunctionBuilder<'_>,
        environment: &ExprEnvironment<'_>,
    ) -> Result<cranelift_codegen::ir::Value, CodegenError> {
        match &expr.value {
            ExprKind::Integer(value) => Ok(builder.ins().iconst(types::I32, *value)),
            ExprKind::Real(value) => Ok(builder.ins().f64const(Ieee64::with_float(*value))),
            ExprKind::Boolean(value) => Ok(builder.ins().iconst(types::I8, i64::from(*value))),
            ExprKind::Variable(name) => Ok(environment.variables[name]),
            ExprKind::List(elements) => {
                let mut values = Vec::with_capacity(elements.len());
                for element in elements {
                    values.push(Self::compile_expr(element, builder, environment)?);
                }
                let flags = cranelift_codegen::ir::MemFlagsData::new();
                // A list is a chain of cons cells ending in nil; elements
                // are stored as uniform words.
                let mut list = builder.ins().iconst(types::I64, value::NIL);
                for value in values.into_iter().rev() {
                    let head = match builder.func.dfg.value_type(value) {
                        types::I32 | types::I8 => {
                            let wide = if builder.func.dfg.value_type(value) == types::I32 {
                                builder.ins().sextend(types::I64, value)
                            } else {
                                builder.ins().uextend(types::I64, value)
                            };
                            let shifted = builder.ins().ishl_imm_u(wide, 1);
                            builder.ins().bor_imm_u(shifted, 1)
                        }
                        types::F64 => {
                            let block = Self::allocate(builder, environment, 1, value::KIND_REAL);
                            builder.ins().store(flags, value, block, 8);
                            block
                        }
                        _ => value,
                    };
                    let cell = Self::allocate(builder, environment, 2, value::KIND_RECORD);
                    builder.ins().store(flags, head, cell, 8);
                    builder.ins().store(flags, list, cell, 16);
                    list = cell;
                }
                Ok(list)
            }
            ExprKind::If(condition, consequent, alternative) => {
                let condition = Self::compile_expr(condition, builder, environment)?;
                let then_block = builder.create_block();
                let else_block = builder.create_block();
                let merge_block = builder.create_block();
                builder
                    .ins()
                    .brif(condition, then_block, &[], else_block, &[]);
                builder.seal_block(then_block);
                builder.seal_block(else_block);

                builder.switch_to_block(then_block);
                let then_value = Self::compile_expr(consequent, builder, environment)?;
                let result_type = builder.func.dfg.value_type(then_value);
                let result = builder.append_block_param(merge_block, result_type);
                let then_arg = then_value.into();
                builder.ins().jump(merge_block, &[then_arg]);

                builder.switch_to_block(else_block);
                let else_value = Self::compile_expr(alternative, builder, environment)?;
                if builder.func.dfg.value_type(else_value) != result_type {
                    return Err(CodegenError::Message(
                        "conditional branches must have the same type".into(),
                    ));
                }
                let else_arg = else_value.into();
                builder.ins().jump(merge_block, &[else_arg]);

                builder.seal_block(merge_block);
                builder.switch_to_block(merge_block);
                Ok(result)
            }
            ExprKind::Greater(lhs, rhs)
            | ExprKind::GreaterEqual(lhs, rhs)
            | ExprKind::Less(lhs, rhs)
            | ExprKind::LessEqual(lhs, rhs)
            | ExprKind::Equal(lhs, rhs)
            | ExprKind::NotEqual(lhs, rhs) => {
                let lhs = Self::compile_expr(lhs, builder, environment)?;
                let rhs = Self::compile_expr(rhs, builder, environment)?;
                let operator = match &expr.value {
                    ExprKind::Greater(_, _) => ComparisonOperator::Greater,
                    ExprKind::GreaterEqual(_, _) => ComparisonOperator::GreaterEqual,
                    ExprKind::Less(_, _) => ComparisonOperator::Less,
                    ExprKind::LessEqual(_, _) => ComparisonOperator::LessEqual,
                    ExprKind::Equal(_, _) => ComparisonOperator::Equal,
                    ExprKind::NotEqual(_, _) => ComparisonOperator::NotEqual,
                    _ => unreachable!(),
                };
                let type_of = |value| match builder.func.dfg.value_type(value) {
                    types::I32 => Ok(Type::Integer),
                    types::F64 => Ok(Type::Real),
                    types::I8 => Ok(Type::Boolean),
                    _ => Err(CodegenError::Message(
                        "unsupported comparison operand type".into(),
                    )),
                };
                let result_type = sema::comparison_result(operator, type_of(lhs)?, type_of(rhs)?)
                    .map_err(|error| CodegenError::Message(error.to_string()))?;
                let comparison = match (operator, result_type, builder.func.dfg.value_type(lhs)) {
                    (ComparisonOperator::Greater, Type::Boolean, types::I32) => {
                        builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs)
                    }
                    (ComparisonOperator::GreaterEqual, Type::Boolean, types::I32) => builder
                        .ins()
                        .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs),
                    (ComparisonOperator::Less, Type::Boolean, types::I32) => {
                        builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs)
                    }
                    (ComparisonOperator::LessEqual, Type::Boolean, types::I32) => {
                        builder.ins().icmp(IntCC::SignedLessThanOrEqual, lhs, rhs)
                    }
                    (
                        ComparisonOperator::Equal | ComparisonOperator::NotEqual,
                        Type::Boolean,
                        types::I32 | types::I8,
                    ) => {
                        let condition = if operator == ComparisonOperator::Equal {
                            IntCC::Equal
                        } else {
                            IntCC::NotEqual
                        };
                        builder.ins().icmp(condition, lhs, rhs)
                    }
                    (ComparisonOperator::Greater, Type::Boolean, types::F64) => {
                        builder.ins().fcmp(FloatCC::GreaterThan, lhs, rhs)
                    }
                    (ComparisonOperator::GreaterEqual, Type::Boolean, types::F64) => {
                        builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lhs, rhs)
                    }
                    (ComparisonOperator::Less, Type::Boolean, types::F64) => {
                        builder.ins().fcmp(FloatCC::LessThan, lhs, rhs)
                    }
                    (ComparisonOperator::LessEqual, Type::Boolean, types::F64) => {
                        builder.ins().fcmp(FloatCC::LessThanOrEqual, lhs, rhs)
                    }
                    _ => unreachable!(),
                };
                Ok(comparison)
            }
            ExprKind::Add(lhs, rhs)
            | ExprKind::Subtract(lhs, rhs)
            | ExprKind::Multiply(lhs, rhs)
            | ExprKind::Divide(lhs, rhs)
            | ExprKind::IntDivide(lhs, rhs) => {
                let lhs = Self::compile_expr(lhs, builder, environment)?;
                let rhs = Self::compile_expr(rhs, builder, environment)?;
                let operator = match &expr.value {
                    ExprKind::Add(_, _) => ArithmeticOperator::Add,
                    ExprKind::Subtract(_, _) => ArithmeticOperator::Subtract,
                    ExprKind::Multiply(_, _) => ArithmeticOperator::Multiply,
                    ExprKind::Divide(_, _) => ArithmeticOperator::Divide,
                    ExprKind::IntDivide(_, _) => ArithmeticOperator::IntDivide,
                    _ => unreachable!(),
                };
                let type_of = |value| match builder.func.dfg.value_type(value) {
                    types::I32 => Ok(Type::Integer),
                    types::F64 => Ok(Type::Real),
                    _ => Err(CodegenError::Message(
                        "unsupported arithmetic operand type".into(),
                    )),
                };
                let result_type = sema::arithmetic_result(operator, type_of(lhs)?, type_of(rhs)?)
                    .map_err(|error| CodegenError::Message(error.to_string()))?;
                let instruction = match (operator, result_type) {
                    (ArithmeticOperator::Add, Type::Integer) => builder.ins().iadd(lhs, rhs),
                    (ArithmeticOperator::Subtract, Type::Integer) => builder.ins().isub(lhs, rhs),
                    (ArithmeticOperator::Multiply, Type::Integer) => builder.ins().imul(lhs, rhs),
                    (ArithmeticOperator::IntDivide, Type::Integer) => builder.ins().sdiv(lhs, rhs),
                    (ArithmeticOperator::Add, Type::Real) => builder.ins().fadd(lhs, rhs),
                    (ArithmeticOperator::Subtract, Type::Real) => builder.ins().fsub(lhs, rhs),
                    (ArithmeticOperator::Multiply, Type::Real) => builder.ins().fmul(lhs, rhs),
                    (ArithmeticOperator::Divide, Type::Real) => builder.ins().fdiv(lhs, rhs),
                    _ => unreachable!(),
                };
                Ok(instruction)
            }
            _ => Err(CodegenError::Message(
                "expected an integer or real expression".into(),
            )),
        }
    }

    /// A heap block of `fields` words after its header.
    fn allocate(
        builder: &mut FunctionBuilder<'_>,
        environment: &ExprEnvironment<'_>,
        fields: i64,
        kind: i64,
    ) -> cranelift_codegen::ir::Value {
        let size = builder
            .ins()
            .iconst(environment.pointer_type, (fields + 1) * 8);
        let call = builder.ins().call(environment.allocator, &[size]);
        let block = builder.inst_results(call)[0];
        let header = builder
            .ins()
            .iconst(types::I64, value::header(fields, kind));
        builder
            .ins()
            .store(cranelift_codegen::ir::MemFlagsData::new(), header, block, 0);
        block
    }

    fn print_timings(&self, function: &FunctionBuild, link: Duration, total: Duration) {
        if self.timings {
            println!(
                "== Timings ==\nfrontend: {:?}\noptimization: {:?}\ncodegen: {:?}\nlink: {:?}\ntotal: {:?}",
                function.frontend, function.optimization, function.codegen, link, total
            );
        }
    }
}
