use std::io::{self, BufRead, Write};

use crate::codegen::{Codegen, CodegenOptions, OptLevel, Symbols};
use crate::runtime;
use nassau::core;
use nassau::printing::ReplValue;
use nassau::session::{Backend, Execution, Session};
use nassau::value;

#[derive(Clone, Copy)]
struct NativeValue(i64);

impl ReplValue for NativeValue {
    fn is_boxed(&self) -> bool {
        self.0 & 1 == 0
    }
    fn immediate(&self) -> i64 {
        self.0
    }
    fn field(&self, index: usize) -> Self {
        Self(unsafe { *((self.0 as *const i64).add(index + 1)) })
    }
    fn length(&self) -> usize {
        unsafe { (*(self.0 as *const i64) >> 8) as usize }
    }
    fn bytes(&self) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts((self.0 as *const u8).add(8), self.length()).to_vec() }
    }
    fn real(&self) -> f64 {
        f64::from_bits(self.field(0).0 as u64)
    }
    fn builtin(&self, name: &str) -> bool {
        value::builtin_exception(name).is_some_and(|id| runtime::builtin_exception(id) == self.0)
    }
    fn with_root<T>(&self, action: impl FnOnce() -> T) -> T {
        runtime::with_root(self.0, action)
    }
}

struct NativeBackend {
    codegen: Codegen,
    module: cranelift_jit::JITModule,
    symbols: Symbols,
}

impl Backend for NativeBackend {
    type Value = NativeValue;
    fn execute(&mut self, module: core::Module) -> miette::Result<Execution<NativeValue>> {
        let entry = self
            .codegen
            .compile_jit_chunk(&mut self.module, &mut self.symbols, &module)
            .map_err(miette::Report::msg)?;
        let status = entry();
        Ok(match runtime::take_uncaught() {
            Some(exception) => Execution::Raised(NativeValue(exception)),
            None => Execution::Returned(NativeValue(value::tagged(status as i64))),
        })
    }
    fn global(&self, id: core::GlobalId) -> Option<NativeValue> {
        Codegen::global_address(&self.module, &self.symbols, id)
            .map(|address| NativeValue(unsafe { address.read() } as i64))
    }
    fn reset(&mut self) -> miette::Result<()> {
        let module = self.codegen.new_jit_module().map_err(miette::Report::msg)?;
        runtime::reset_repl_roots();
        self.symbols = Symbols::default();
        let previous = std::mem::replace(&mut self.module, module);
        // old code is idle and unreachable after the session roots are cleared
        unsafe { previous.free_memory() };
        Ok(())
    }
    fn retain_globals(&mut self, roots: &[core::GlobalId]) {
        let addresses = roots
            .iter()
            .filter_map(|id| Codegen::global_address(&self.module, &self.symbols, *id))
            .map(|address| address as usize)
            .collect::<Vec<_>>();
        runtime::replace_global_roots(&addresses);
    }
}

pub fn run(
    interpret: bool,
    opt_level: OptLevel,
    debug_passes: bool,
    dump_ir: bool,
    dump_optimized_ir: bool,
    verify: bool,
    stats: bool,
) -> miette::Result<()> {
    if interpret {
        return terminal(Session::default());
    }
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
    let module = codegen.new_jit_module().map_err(miette::Report::msg)?;
    runtime::enter_repl();
    terminal(Session::new(NativeBackend {
        codegen,
        module,
        symbols: Symbols::default(),
    }))
}

fn terminal<B: Backend>(mut session: Session<B>) -> miette::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut line = String::new();
    let mut chunk = String::new();
    loop {
        if chunk.is_empty() {
            print!("nassau> ");
            io::stdout().flush().map_err(miette::Report::msg)?;
        }
        line.clear();
        let bytes = input.read_line(&mut line).map_err(miette::Report::msg)?;
        chunk.push_str(&line);
        if bytes != 0 && !chunk.trim_end().ends_with(';') {
            continue;
        }
        if !chunk.trim().is_empty() {
            let response = session.submit(&chunk);
            if response.clear {
                io::stdout()
                    .write_all(b"\x1b[2J\x1b[H")
                    .map_err(miette::Report::msg)?;
            }
            io::stdout()
                .write_all(&response.output)
                .map_err(miette::Report::msg)?;
            io::stderr()
                .write_all(response.diagnostics.as_bytes())
                .map_err(miette::Report::msg)?;
            if let Some(status) = response.exit {
                std::process::exit(i32::from(status));
            }
        }
        chunk.clear();
        if bytes == 0 {
            break;
        }
    }
    Ok(())
}
