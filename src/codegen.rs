//! Translates the core IR to Cranelift, for an object file or the JIT.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

use clap::ValueEnum;
use cranelift_codegen::ir::{
    AbiParam, BlockArg, FuncRef, InstBuilder, MemFlagsData, Signature, Value,
    condcodes::{FloatCC, IntCC},
    types,
};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::Configurable;
use cranelift_codegen::{self, settings};
use cranelift_control::ControlPlane;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, DataId, FuncId, Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::core::{self, Atom, Callee, Failure, FnId, GlobalId, Op, Prim, Stmt, Term};
use crate::error::CodegenError;
use crate::runtime;
use crate::value;

mod helpers;

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

/// The Cranelift functions and data already declared for IR functions and
/// globals; the REPL keeps them from one chunk to the next.
#[derive(Default)]
pub struct Symbols {
    functions: HashMap<FnId, FuncId>,
    globals: HashMap<GlobalId, DataId>,
    /// How many anonymous data objects (literals) exist.
    data: usize,
    helpers: HashMap<&'static str, FuncId>,
}

struct FunctionBuild {
    context: cranelift_codegen::Context,
    frontend: Duration,
    optimization: Duration,
    codegen: Duration,
}

fn backend(error: impl ToString) -> CodegenError {
    CodegenError::Backend(error.to_string())
}

/// The signature of every compiled SML function: the environment, then the
/// arguments, all words.
fn sml_signature(params: usize) -> Signature {
    let mut signature = Signature::new(CallConv::Tail);
    signature
        .params
        .extend((0..params).map(|_| AbiParam::new(types::I64)));
    signature.returns.push(AbiParam::new(types::I64));
    signature
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
            .map_err(backend)?;
        // Cranelift's tail calls rely on frame pointers.
        flag_builder
            .set("preserve_frame_pointers", "true")
            .map_err(backend)?;
        if jit {
            flag_builder
                .set("use_colocated_libcalls", "false")
                .map_err(backend)?;
            if cfg!(target_arch = "x86_64") {
                flag_builder.set("is_pic", "true").map_err(backend)?;
            }
        }
        cranelift_native::builder()
            .map_err(backend)?
            .finish(settings::Flags::new(flag_builder))
            .map_err(backend)
    }

    pub fn new_jit_module(&self) -> Result<JITModule, CodegenError> {
        let mut builder = JITBuilder::with_isa(self.isa(true)?, default_libcall_names());
        for (name, address) in runtime::symbols() {
            builder.symbol(name, address);
        }
        Ok(JITModule::new(builder))
    }

    /// Compiles a REPL chunk and returns its entry function.
    pub fn compile_jit_chunk(
        &self,
        jit: &mut JITModule,
        symbols: &mut Symbols,
        module: &core::Module,
    ) -> Result<extern "C" fn() -> i32, CodegenError> {
        let (entry, _) = self.define_module(jit, symbols, module)?;
        jit.finalize_definitions().map_err(backend)?;
        let code = jit.get_finalized_function(entry);
        // SAFETY: the entry function takes no arguments and returns an i32
        // in the platform's C calling convention.
        Ok(unsafe { std::mem::transmute::<*const u8, extern "C" fn() -> i32>(code) })
    }

    /// The address of a global's cell, once its chunk is compiled.
    pub fn global_address(
        jit: &JITModule,
        symbols: &Symbols,
        global: GlobalId,
    ) -> Option<*const u64> {
        let data = symbols.globals.get(&global)?;
        Some(jit.get_finalized_data(*data).0.cast())
    }

    pub fn compile(&self, module: &core::Module, output: &Path) -> Result<(), CodegenError> {
        let total_start = Instant::now();
        let isa = self.isa(false)?;
        let object_builder =
            ObjectBuilder::new(isa, "nassau", default_libcall_names()).map_err(backend)?;
        let mut object_module = ObjectModule::new(object_builder);
        let (_, functions) =
            self.define_module(&mut object_module, &mut Symbols::default(), module)?;

        if self.debug_passes || self.asm {
            let mut disassembly = String::new();
            for function in &functions {
                let listing = function
                    .context
                    .compiled_code()
                    .and_then(|compiled| compiled.vcode.as_deref())
                    .ok_or_else(|| {
                        CodegenError::Message(
                            "target does not provide a textual assembly listing".into(),
                        )
                    })?;
                disassembly.push_str(listing);
            }
            if self.debug_passes {
                println!("== Cranelift machine instructions ==\n{disassembly}");
            }
            if self.asm {
                fs::write(output, disassembly)
                    .map_err(|error| CodegenError::Io(error.to_string()))?;
                self.print_timings(&functions, Duration::ZERO, total_start.elapsed());
                return Ok(());
            }
        }

        let object = object_module.finish().emit().map_err(backend)?;
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
        let runtime_path = env::temp_dir().join(format!("nassau-runtime-{}.a", std::process::id()));
        fs::write(&runtime_path, runtime::ARCHIVE)
            .map_err(|error| CodegenError::Io(error.to_string()))?;
        let linker = env::var("NASSAU_CC").unwrap_or_else(|_| "cc".to_string());
        let link_start = Instant::now();
        let link_result = Command::new(&linker)
            .args(["-o"])
            .arg(output)
            .arg(&object_path)
            .arg(&runtime_path)
            .output()
            .map_err(|error| CodegenError::Tool(format!("failed to invoke {linker}: {error}")));
        let _ = fs::remove_file(&object_path);
        let _ = fs::remove_file(&runtime_path);
        let link_result = link_result?;
        if !link_result.status.success() {
            return Err(CodegenError::Linker(
                String::from_utf8_lossy(&link_result.stderr)
                    .trim()
                    .to_string(),
            ));
        }
        self.print_timings(&functions, link_start.elapsed(), total_start.elapsed());
        Ok(())
    }

    /// Declares and defines everything `module` introduces; returns its
    /// entry function.
    fn define_module<M: Module>(
        &self,
        target: &mut M,
        symbols: &mut Symbols,
        module: &core::Module,
    ) -> Result<(FuncId, Vec<FunctionBuild>), CodegenError> {
        let mut builds = self.define_helpers(target, symbols, module)?;
        for (global, name) in &module.globals {
            let id = target
                .declare_data(
                    &format!("nassau_global_{global}_{}", symbol_name(name)),
                    Linkage::Local,
                    true,
                    false,
                )
                .map_err(backend)?;
            let mut data = DataDescription::new();
            data.define(value::NIL.to_le_bytes().to_vec().into_boxed_slice());
            data.set_align(8);
            target.define_data(id, &data).map_err(backend)?;
            symbols.globals.insert(*global, id);
        }
        for function in &module.functions {
            let id = target
                .declare_function(
                    &format!("nassau_f{}_{}", function.id, symbol_name(&function.name)),
                    Linkage::Local,
                    &sml_signature(function.params.len()),
                )
                .map_err(backend)?;
            symbols.functions.insert(function.id, id);
        }
        let mut entry_signature = target.make_signature();
        entry_signature.returns.push(AbiParam::new(types::I32));
        let entry = target
            .declare_function(&module.entry.name, Linkage::Export, &entry_signature)
            .map_err(backend)?;
        for function in &module.functions {
            let id = symbols.functions[&function.id];
            builds.push(self.define_function(
                target,
                symbols,
                function,
                id,
                false,
                &module.file,
            )?);
        }
        builds.push(self.define_function(
            target,
            symbols,
            &module.entry,
            entry,
            true,
            &module.file,
        )?);
        Ok((entry, builds))
    }

    fn define_function<M: Module>(
        &self,
        target: &mut M,
        symbols: &mut Symbols,
        function: &core::Function,
        id: FuncId,
        entry: bool,
        file: &str,
    ) -> Result<FunctionBuild, CodegenError> {
        let total_start = Instant::now();
        let frontend_config = target.isa().frontend_config();
        let mut context = target.make_context();
        context.func.signature = if entry {
            let mut signature = target.make_signature();
            signature.returns.push(AbiParam::new(types::I32));
            signature
        } else {
            sml_signature(function.params.len())
        };
        let mut builder_context = FunctionBuilderContext::new();
        let builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let mut translator = Translator {
            target,
            symbols,
            builder,
            vars: HashMap::new(),
            blocks: Vec::new(),
            imports: HashMap::new(),
            entry,
            file,
            handler: None,
            landings: Vec::new(),
        };
        translator.function(function)?;
        translator.builder.seal_all_blocks();
        translator.builder.finalize(frontend_config);
        let frontend = total_start.elapsed();

        self.finish_function(target, id, context, frontend)
    }

    fn finish_function<M: Module>(
        &self,
        target: &mut M,
        id: FuncId,
        mut context: cranelift_codegen::Context,
        frontend: Duration,
    ) -> Result<FunctionBuild, CodegenError> {
        let dump_ir = self.debug_passes || self.dump_ir;
        let dump_optimized_ir = self.debug_passes || self.dump_optimized_ir;
        let needs_manual_optimization = dump_optimized_ir || self.verify || self.stats;
        if self.verify {
            cranelift_codegen::verify_function(&context.func, target.isa()).map_err(backend)?;
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
                .optimize(target.isa(), &mut ControlPlane::default())
                .map_err(backend)?;
        }
        let optimization = optimization_start.elapsed();
        if self.verify && needs_manual_optimization {
            cranelift_codegen::verify_function(&context.func, target.isa()).map_err(backend)?;
        }
        if dump_optimized_ir {
            println!(
                "== Cranelift IR after optimization ==\n{}",
                context.func.display()
            );
        }
        context.set_disasm(self.debug_passes || self.asm);
        let codegen_start = Instant::now();
        target
            .define_function(id, &mut context)
            .map_err(|error| CodegenError::Backend(format!("{error:?}")))?;
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
            context,
            frontend,
            optimization,
            codegen,
        })
    }

    fn print_timings(&self, functions: &[FunctionBuild], link: Duration, total: Duration) {
        if self.timings {
            let frontend: Duration = functions.iter().map(|function| function.frontend).sum();
            let optimization: Duration =
                functions.iter().map(|function| function.optimization).sum();
            let codegen: Duration = functions.iter().map(|function| function.codegen).sum();
            println!(
                "== Timings ==\nfrontend: {frontend:?}\noptimization: {optimization:?}\ncodegen: {codegen:?}\nlink: {link:?}\ntotal: {total:?}",
            );
        }
    }
}

/// A source name reduced to the characters a symbol may hold.
fn symbol_name(name: &str) -> String {
    name.chars()
        .map(|char| {
            if char.is_ascii_alphanumeric() {
                char
            } else {
                '_'
            }
        })
        .collect()
}

struct Translator<'a, M: Module> {
    target: &'a mut M,
    symbols: &'a mut Symbols,
    builder: FunctionBuilder<'a>,
    vars: HashMap<core::Var, Value>,
    blocks: Vec<cranelift_codegen::ir::Block>,
    imports: HashMap<&'static str, FuncRef>,
    /// Whether this is a chunk's entry, which returns an exit status.
    entry: bool,
    /// The source file's name, for reporting exceptions.
    file: &'a str,
    /// The handler of the block being translated.
    handler: Option<core::BlockId>,
    /// The block a raised exception leaves through, for each handler (or
    /// none) that has one.
    landings: Vec<(Option<core::BlockId>, cranelift_codegen::ir::Block)>,
}

impl<M: Module> Translator<'_, M> {
    fn function(&mut self, function: &core::Function) -> Result<(), CodegenError> {
        for block in &function.blocks {
            let created = self.builder.create_block();
            for param in &block.params {
                let value = self.builder.append_block_param(created, types::I64);
                self.vars.insert(*param, value);
            }
            self.blocks.push(created);
        }
        let start = self.blocks[0];
        self.builder.append_block_params_for_function_params(start);
        let params = self.builder.block_params(start).to_vec();
        for (var, value) in function.params.iter().zip(params) {
            self.vars.insert(*var, value);
        }
        for (block, created) in function.blocks.iter().zip(self.blocks.clone()) {
            self.builder.switch_to_block(created);
            self.handler = block.handler;
            for stmt in &block.stmts {
                self.stmt(stmt)?;
            }
            self.term(&block.term)?;
        }
        for (handler, landing) in self.landings.clone() {
            self.builder.switch_to_block(landing);
            match handler {
                Some(handler) => {
                    let state = self.raised_address()?;
                    let exception =
                        self.builder
                            .ins()
                            .load(types::I64, MemFlagsData::trusted(), state, 0);
                    let empty = self.word(value::RAISED);
                    self.store(empty, state, 0);
                    self.builder
                        .ins()
                        .jump(self.blocks[handler], &[exception.into()]);
                }
                None if self.entry => {
                    let status = self
                        .call_c("nassau_uncaught", &[], Some(types::I32), &[])?
                        .expect("nassau_uncaught returns an exit status");
                    self.builder.ins().return_(&[status]);
                }
                None => {
                    let raised = self.word(value::RAISED);
                    self.builder.ins().return_(&[raised]);
                }
            }
        }
        Ok(())
    }

    /// Where an exception raised in the current block goes: into its
    /// handler, or out of the function.
    fn landing(&mut self) -> cranelift_codegen::ir::Block {
        if let Some((_, landing)) = self
            .landings
            .iter()
            .find(|(handler, _)| *handler == self.handler)
        {
            return *landing;
        }
        let landing = self.builder.create_block();
        self.builder.set_cold_block(landing);
        self.landings.push((self.handler, landing));
        landing
    }

    fn word(&mut self, word: i64) -> Value {
        self.builder.ins().iconst(types::I64, word)
    }

    fn atom(&mut self, atom: &Atom) -> Result<Value, CodegenError> {
        Ok(match atom {
            Atom::Var(var) => self.vars[var],
            Atom::Word(word) => self.word(*word),
            Atom::Real(real) => {
                let mut bytes = value::header(1, value::KIND_REAL).to_le_bytes().to_vec();
                bytes.extend(real.to_le_bytes());
                self.data(bytes)?
            }
            Atom::String(text) => {
                let length = text.chars().count() as i64;
                let mut bytes = value::header(length, value::KIND_STRING)
                    .to_le_bytes()
                    .to_vec();
                bytes.extend(text.chars().map(|ch| ch as u8));
                bytes.push(0);
                while !bytes.len().is_multiple_of(8) {
                    bytes.push(0);
                }
                self.data(bytes)?
            }
        })
    }

    fn atoms(&mut self, atoms: &[Atom]) -> Result<Vec<Value>, CodegenError> {
        atoms.iter().map(|atom| self.atom(atom)).collect()
    }

    /// The address of a new read-only data object holding `bytes`.
    fn data(&mut self, bytes: Vec<u8>) -> Result<Value, CodegenError> {
        self.symbols.data += 1;
        let id = self
            .target
            .declare_data(
                &format!("nassau_data_{}", self.symbols.data),
                Linkage::Local,
                false,
                false,
            )
            .map_err(backend)?;
        let mut data = DataDescription::new();
        data.define(bytes.into_boxed_slice());
        data.set_align(8);
        self.target.define_data(id, &data).map_err(backend)?;
        Ok(self.data_address(id))
    }

    fn data_address(&mut self, id: DataId) -> Value {
        let global = self.target.declare_data_in_func(id, self.builder.func);
        self.builder.ins().symbol_value(types::I64, global)
    }

    /// A C function, declared on first use.
    fn import(
        &mut self,
        name: &'static str,
        params: &[types::Type],
        returns: Option<types::Type>,
    ) -> Result<FuncRef, CodegenError> {
        if let Some(found) = self.imports.get(name) {
            return Ok(*found);
        }
        let mut signature = self.target.make_signature();
        signature
            .params
            .extend(params.iter().map(|param| AbiParam::new(*param)));
        signature.returns.extend(returns.map(AbiParam::new));
        let id = if let Some(id) = self.symbols.helpers.get(name) {
            *id
        } else {
            self.target
                .declare_function(name, Linkage::Import, &signature)
                .map_err(backend)?
        };
        let func = self.target.declare_func_in_func(id, self.builder.func);
        self.imports.insert(name, func);
        Ok(func)
    }

    fn call_c(
        &mut self,
        name: &'static str,
        params: &[types::Type],
        returns: Option<types::Type>,
        args: &[Value],
    ) -> Result<Option<Value>, CodegenError> {
        let func = self.import(name, params, returns)?;
        let call = self.builder.ins().call(func, args);
        Ok(self.builder.inst_results(call).first().copied())
    }

    /// A heap block with room for `fields` words after its header.
    fn allocate(&mut self, fields: usize, kind: i64) -> Result<Value, CodegenError> {
        let size = self.word(fields as i64);
        let block = self
            .call_c("nassau_alloc", &[types::I64], Some(types::I64), &[size])?
            .expect("nassau_alloc returns a pointer");
        let header = self.word(value::header(fields as i64, kind));
        self.store(header, block, 0);
        Ok(block)
    }

    fn load(&mut self, block: Value, field: usize) -> Value {
        self.builder.ins().load(
            types::I64,
            MemFlagsData::trusted(),
            block,
            (field as i32 + 1) * 8,
        )
    }

    /// Stores `value` at byte `offset` of `block`.
    fn store(&mut self, value: Value, block: Value, offset: i32) {
        self.builder
            .ins()
            .store(MemFlagsData::trusted(), value, block, offset);
    }

    fn untag(&mut self, word: Value) -> Value {
        self.builder.ins().sshr_imm_u(word, 1)
    }

    fn tag(&mut self, integer: Value) -> Value {
        let shifted = self.builder.ins().ishl_imm_u(integer, 1);
        self.builder.ins().bor_imm_u(shifted, 1)
    }

    /// The `bool` word for a Cranelift condition.
    fn boolean(&mut self, condition: Value) -> Value {
        let wide = self.builder.ins().uextend(types::I64, condition);
        self.tag(wide)
    }

    fn real(&mut self, boxed: Value) -> Value {
        self.builder
            .ins()
            .load(types::F64, MemFlagsData::trusted(), boxed, 8)
    }

    fn box_real(&mut self, real: Value) -> Result<Value, CodegenError> {
        let block = self.allocate(1, value::KIND_REAL)?;
        self.builder
            .ins()
            .store(MemFlagsData::trusted(), real, block, 8);
        Ok(block)
    }

    fn prim(&mut self, prim: Prim, args: &[Value]) -> Result<Value, CodegenError> {
        Ok(match prim {
            // On tagged words 2a+1 and 2b+1: (2a+1) + (2b+1) - 1 = 2(a+b)+1.
            Prim::IntAdd => {
                let sum = self.builder.ins().iadd(args[0], args[1]);
                let sum = self.builder.ins().iadd_imm_s(sum, -1);
                self.check_overflow(sum)?
            }
            Prim::IntSub => {
                let difference = self.builder.ins().isub(args[0], args[1]);
                let difference = self.builder.ins().iadd_imm_s(difference, 1);
                self.check_overflow(difference)?
            }
            // a * (2b) + 1; the product of two 31-bit ints fits in 64 bits.
            Prim::IntMul => {
                let lhs = self.untag(args[0]);
                let rhs = self.builder.ins().iadd_imm_s(args[1], -1);
                let product = self.builder.ins().imul(lhs, rhs);
                let product = self.builder.ins().iadd_imm_s(product, 1);
                self.check_overflow(product)?
            }
            Prim::IntDiv | Prim::IntMod => {
                // The third argument is where SML/NJ reports `Div`.
                let zero = self
                    .builder
                    .ins()
                    .icmp_imm_s(IntCC::Equal, args[1], value::tagged(0));
                self.raise_if(zero, "Div", args[2])?;
                let lhs = self.untag(args[0]);
                let rhs = self.untag(args[1]);
                let quotient = self.builder.ins().sdiv(lhs, rhs);
                let remainder = self.builder.ins().srem(lhs, rhs);
                // SML rounds toward negative infinity: adjust when the
                // remainder is nonzero and its sign differs from the divisor's.
                let inexact = self.builder.ins().icmp_imm_s(IntCC::NotEqual, remainder, 0);
                let signs = self.builder.ins().bxor(remainder, rhs);
                let differ = self
                    .builder
                    .ins()
                    .icmp_imm_s(IntCC::SignedLessThan, signs, 0);
                let adjust = self.builder.ins().band(inexact, differ);
                let adjust = self.builder.ins().uextend(types::I64, adjust);
                let result = if prim == Prim::IntDiv {
                    self.builder.ins().isub(quotient, adjust)
                } else {
                    let correction = self.builder.ins().imul(adjust, rhs);
                    self.builder.ins().iadd(remainder, correction)
                };
                // minInt div ~1 overflows.
                let result = self.tag(result);
                self.check_overflow(result)?
            }
            Prim::IntNeg => {
                let two = self.word(2);
                let negated = self.builder.ins().isub(two, args[0]);
                self.check_overflow(negated)?
            }
            Prim::IntLt | Prim::IntLe | Prim::IntGt | Prim::IntGe => {
                let condition = match prim {
                    Prim::IntLt => IntCC::SignedLessThan,
                    Prim::IntLe => IntCC::SignedLessThanOrEqual,
                    Prim::IntGt => IntCC::SignedGreaterThan,
                    _ => IntCC::SignedGreaterThanOrEqual,
                };
                let compared = self.builder.ins().icmp(condition, args[0], args[1]);
                self.boolean(compared)
            }
            Prim::StringLt | Prim::StringLe | Prim::StringGt | Prim::StringGe => {
                let left_header =
                    self.builder
                        .ins()
                        .load(types::I64, MemFlagsData::trusted(), args[0], 0);
                let right_header =
                    self.builder
                        .ins()
                        .load(types::I64, MemFlagsData::trusted(), args[1], 0);
                let left_length = self.builder.ins().ushr_imm_u(left_header, 8);
                let right_length = self.builder.ins().ushr_imm_u(right_header, 8);
                let shorter =
                    self.builder
                        .ins()
                        .icmp(IntCC::UnsignedLessThan, left_length, right_length);
                let length = self
                    .builder
                    .ins()
                    .select(shorter, left_length, right_length);
                let left = self.builder.ins().iadd_imm_s(args[0], 8);
                let right = self.builder.ins().iadd_imm_s(args[1], 8);
                let compared = self.builder.call_memcmp(
                    self.target.isa().frontend_config(),
                    left,
                    right,
                    length,
                );
                let compared = self.builder.ins().sextend(types::I64, compared);
                let equal = self.builder.ins().icmp_imm_s(IntCC::Equal, compared, 0);
                let lengths = self.builder.ins().isub(left_length, right_length);
                let order = self.builder.ins().select(equal, lengths, compared);
                let condition = match prim {
                    Prim::StringLt => IntCC::SignedLessThan,
                    Prim::StringLe => IntCC::SignedLessThanOrEqual,
                    Prim::StringGt => IntCC::SignedGreaterThan,
                    _ => IntCC::SignedGreaterThanOrEqual,
                };
                let result = self.builder.ins().icmp_imm_s(condition, order, 0);
                self.boolean(result)
            }
            Prim::WordEq | Prim::WordNe => {
                let condition = if prim == Prim::WordEq {
                    IntCC::Equal
                } else {
                    IntCC::NotEqual
                };
                let compared = self.builder.ins().icmp(condition, args[0], args[1]);
                self.boolean(compared)
            }
            Prim::RealAdd | Prim::RealSub | Prim::RealMul | Prim::RealDiv => {
                let lhs = self.real(args[0]);
                let rhs = self.real(args[1]);
                let result = match prim {
                    Prim::RealAdd => self.builder.ins().fadd(lhs, rhs),
                    Prim::RealSub => self.builder.ins().fsub(lhs, rhs),
                    Prim::RealMul => self.builder.ins().fmul(lhs, rhs),
                    _ => self.builder.ins().fdiv(lhs, rhs),
                };
                self.box_real(result)?
            }
            Prim::RealNeg => {
                let real = self.real(args[0]);
                let negated = self.builder.ins().fneg(real);
                self.box_real(negated)?
            }
            Prim::RealLt | Prim::RealLe | Prim::RealGt | Prim::RealGe => {
                let lhs = self.real(args[0]);
                let rhs = self.real(args[1]);
                let condition = match prim {
                    Prim::RealLt => FloatCC::LessThan,
                    Prim::RealLe => FloatCC::LessThanOrEqual,
                    Prim::RealGt => FloatCC::GreaterThan,
                    _ => FloatCC::GreaterThanOrEqual,
                };
                let compared = self.builder.ins().fcmp(condition, lhs, rhs);
                self.boolean(compared)
            }
            Prim::IsBoxed => {
                let bit = self.builder.ins().band_imm_u(args[0], 1);
                let boxed = self.builder.ins().icmp_imm_s(IntCC::Equal, bit, 0);
                self.boolean(boxed)
            }
            Prim::Print => {
                self.call_c("nassau_print", &[types::I64], Some(types::I64), args)?;
                self.word(value::tagged(0))
            }
            Prim::Concat => self
                .call_c(
                    "nassau_concat",
                    &[types::I64, types::I64],
                    Some(types::I64),
                    args,
                )?
                .expect("nassau_concat returns a string"),
            Prim::IntToString => self
                .call_c(
                    "nassau_int_to_string",
                    &[types::I64],
                    Some(types::I64),
                    args,
                )?
                .expect("nassau_int_to_string returns a string"),
            Prim::Size => {
                let header =
                    self.builder
                        .ins()
                        .load(types::I64, MemFlagsData::trusted(), args[0], 0);
                let length = self.builder.ins().sshr_imm_u(header, 8);
                self.tag(length)
            }
            Prim::Ref => {
                let cell = self.allocate(1, value::KIND_REF)?;
                self.store(args[0], cell, 8);
                cell
            }
            Prim::Assign => {
                self.store(args[1], args[0], 8);
                self.word(value::tagged(0))
            }
            Prim::Exit => {
                self.call_c("nassau_exit", &[types::I64], None, args)?;
                self.word(value::tagged(0))
            }
            Prim::BuiltinException => self
                .call_c("nassau_exception", &[types::I64], Some(types::I64), args)?
                .expect("nassau_exception returns an identity"),
            Prim::Equal | Prim::Unequal => {
                let equal = self
                    .call_c(
                        "nassau_equal",
                        &[types::I64, types::I64],
                        Some(types::I64),
                        args,
                    )?
                    .expect("nassau_equal returns a bool");
                if prim == Prim::Equal {
                    equal
                } else {
                    // Flip the bool's payload bit: 1 <-> 3.
                    self.builder.ins().bxor_imm_u(equal, 2)
                }
            }
        })
    }

    /// Raises the built-in exception `name` when `condition` holds;
    /// `location` is the string naming where.
    fn raise_if(
        &mut self,
        condition: Value,
        name: &str,
        location: Value,
    ) -> Result<(), CodegenError> {
        let raise = self.builder.create_block();
        let next = self.builder.create_block();
        self.builder.set_cold_block(raise);
        self.builder.ins().brif(condition, raise, &[], next, &[]);
        self.builder.switch_to_block(raise);
        self.raise_builtin(name, location)?;
        self.builder.switch_to_block(next);
        Ok(())
    }

    /// Raises the built-in exception `name`, ending the current block.
    fn raise_builtin(&mut self, name: &str, location: Value) -> Result<(), CodegenError> {
        let index = value::builtin_exception(name).expect("a built-in exception");
        let index = self.word(value::tagged(index));
        let identity = self
            .call_c(
                "nassau_exception",
                &[types::I64],
                Some(types::I64),
                &[index],
            )?
            .expect("nassau_exception returns an identity");
        let exception = self.allocate(3, value::KIND_RECORD)?;
        self.store(identity, exception, 8);
        let unit = self.word(value::NIL);
        self.store(unit, exception, 16);
        self.store(location, exception, 24);
        let state = self.raised_address()?;
        self.store(exception, state, 0);
        let landing = self.landing();
        self.builder.ins().jump(landing, &[]);
        Ok(())
    }

    /// Raises `Overflow` unless the tagged int `word` is in range: a 31-bit
    /// int tags to a 32-bit word.
    fn check_overflow(&mut self, word: Value) -> Result<Value, CodegenError> {
        let narrow = self.builder.ins().ireduce(types::I32, word);
        let wide = self.builder.ins().sextend(types::I64, narrow);
        let outside = self.builder.ins().icmp(IntCC::NotEqual, wide, word);
        // SML/NJ reports Overflow at the file, not a position in it.
        let location = self.atom(&Atom::String(format!("<file {}>", self.file)))?;
        self.raise_if(outside, "Overflow", location)?;
        Ok(word)
    }

    fn callee(&mut self, callee: &Callee, args: &[Atom]) -> Result<Call, CodegenError> {
        let mut values = Vec::with_capacity(args.len() + 1);
        let target = match callee {
            Callee::Known(function, env) => {
                values.push(self.atom(env)?);
                let id = self.symbols.functions[function];
                CallTarget::Direct(self.target.declare_func_in_func(id, self.builder.func))
            }
            Callee::Closure(closure) => {
                let closure = self.atom(closure)?;
                values.push(closure);
                let code = self.load(closure, 0);
                let signature = self.builder.import_signature(sml_signature(args.len() + 1));
                CallTarget::Indirect(signature, code)
            }
        };
        values.extend(self.atoms(args)?);
        Ok(Call { target, values })
    }

    fn op(&mut self, op: &Op) -> Result<Value, CodegenError> {
        Ok(match op {
            Op::Atom(atom) => self.atom(atom)?,
            Op::Prim(prim, args) => {
                let args = self.atoms(args)?;
                self.prim(*prim, &args)?
            }
            Op::Record(fields) => {
                let fields = self.atoms(fields)?;
                let block = self.allocate(fields.len(), value::KIND_RECORD)?;
                for (index, field) in fields.into_iter().enumerate() {
                    self.store(field, block, (index as i32 + 1) * 8);
                }
                block
            }
            Op::Select(block, index) => {
                let block = self.atom(block)?;
                self.load(block, *index)
            }
            Op::Global(global) => {
                let address = self.data_address(self.symbols.globals[global]);
                self.builder
                    .ins()
                    .load(types::I64, MemFlagsData::trusted(), address, 0)
            }
            Op::Call(callee, args) => {
                let call = self.callee(callee, args)?;
                let inst = match call.target {
                    CallTarget::Direct(func) => self.builder.ins().call(func, &call.values),
                    CallTarget::Indirect(signature, code) => {
                        self.builder
                            .ins()
                            .call_indirect(signature, code, &call.values)
                    }
                };
                // A function that raises returns `RAISED`, having recorded
                // the exception with the runtime.
                let result = self.builder.inst_results(inst)[0];
                let raised = self
                    .builder
                    .ins()
                    .icmp_imm_s(IntCC::Equal, result, value::RAISED);
                let landing = self.landing();
                let next = self.builder.create_block();
                self.builder.ins().brif(raised, landing, &[], next, &[]);
                self.builder.switch_to_block(next);
                result
            }
        })
    }

    fn stmt(&mut self, stmt: &Stmt) -> Result<(), CodegenError> {
        match stmt {
            Stmt::Let(var, op) => {
                let value = self.op(op)?;
                self.vars.insert(*var, value);
            }
            Stmt::SetGlobal(global, atom) => {
                let value = self.atom(atom)?;
                let address = self.data_address(self.symbols.globals[global]);
                self.store(value, address, 0);
            }
            Stmt::Closures(closures) => {
                // Allocate every closure first, so they can capture each other.
                for closure in closures {
                    let block = self.allocate(closure.captured.len() + 1, value::KIND_CLOSURE)?;
                    self.vars.insert(closure.var, block);
                }
                for closure in closures {
                    let block = self.vars[&closure.var];
                    let id = self.symbols.functions[&closure.code];
                    let func = self.target.declare_func_in_func(id, self.builder.func);
                    let code = self.builder.ins().func_addr(types::I64, func);
                    self.store(code, block, 8);
                    for (index, captured) in closure.captured.iter().enumerate() {
                        let value = self.atom(captured)?;
                        self.store(value, block, (index as i32 + 2) * 8);
                    }
                }
            }
        }
        Ok(())
    }

    fn term(&mut self, term: &Term) -> Result<(), CodegenError> {
        match term {
            Term::Return(atom) => {
                let value = self.atom(atom)?;
                self.ret(value);
            }
            Term::Jump(target, args) => {
                let args: Vec<BlockArg> = self.atoms(args)?.into_iter().map(Into::into).collect();
                self.builder.ins().jump(self.blocks[*target], &args);
            }
            Term::If(condition, then, otherwise) => {
                let condition = self.atom(condition)?;
                let condition =
                    self.builder
                        .ins()
                        .icmp_imm_s(IntCC::NotEqual, condition, value::FALSE);
                self.builder.ins().brif(
                    condition,
                    self.blocks[*then],
                    &[],
                    self.blocks[*otherwise],
                    &[],
                );
            }
            Term::TailCall(callee, args) => {
                debug_assert!(!self.entry, "a chunk's entry makes no tail calls");
                debug_assert!(
                    self.handler.is_none(),
                    "a call under a handler is not a tail call"
                );
                let call = self.callee(callee, args)?;
                match call.target {
                    CallTarget::Direct(func) => {
                        self.builder.ins().return_call(func, &call.values);
                    }
                    CallTarget::Indirect(signature, code) => {
                        self.builder
                            .ins()
                            .return_call_indirect(signature, code, &call.values);
                    }
                }
            }
            Term::Fail(failure, location) => {
                let name = match failure {
                    Failure::Match => "Match",
                    Failure::Bind => "Bind",
                };
                let location = self.atom(&Atom::String(location.clone()))?;
                self.raise_builtin(name, location)?;
            }
            Term::Raise(exception, location) => {
                let exception = self.atom(exception)?;
                let location = match location {
                    Some(location) => self.atom(&Atom::String(location.clone()))?,
                    // A handler passing an exception on keeps its position.
                    None => self.word(0),
                };
                self.raise_exception(exception, location)?;
                let landing = self.landing();
                self.builder.ins().jump(landing, &[]);
            }
        }
        Ok(())
    }

    fn ret(&mut self, value: Value) {
        if self.entry {
            // The entry returns the exit status, an int.
            let status = self.untag(value);
            let status = self.builder.ins().ireduce(types::I32, status);
            self.builder.ins().return_(&[status]);
        } else {
            self.builder.ins().return_(&[value]);
        }
    }
}

enum CallTarget {
    Direct(FuncRef),
    Indirect(cranelift_codegen::ir::SigRef, Value),
}

struct Call {
    target: CallTarget,
    values: Vec<Value>,
}
