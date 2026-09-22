use std::io::{self, BufRead, Write};

use crate::codegen::{Codegen, OptLevel};
use crate::error::CodegenError;
use crate::parser::{Parser, Program};

pub struct Repl {
    codegen: Codegen,
    module: cranelift_jit::JITModule,
    next_chunk: usize,
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
        })
    }

    fn parse(&self, source: &str, chunk: usize) -> miette::Result<Program> {
        let filename = format!("<repl:{chunk}>");
        let named_source = miette::NamedSource::new(filename.clone(), source.to_owned());
        Parser::from_repl_source(source, &filename)
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
            .compile_jit_chunk(&mut self.module, &program, &name)
            .map_err(miette::Report::new)?;
        let function: extern "C" fn() -> i32 = unsafe { std::mem::transmute(function) };
        let result = function();
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
