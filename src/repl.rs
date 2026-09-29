use std::io::{self, BufRead, Write};

use crate::codegen::{Codegen, CodegenOptions, OptLevel, Symbols};
use crate::error::{CodegenError, SourceError};
use crate::infer::{self, Ty};
use crate::lower;
use crate::parser::{Parser, Program};
use crate::value;

pub struct Repl {
    codegen: Codegen,
    module: cranelift_jit::JITModule,
    symbols: Symbols,
    next_chunk: usize,
    types: infer::Session,
    lowering: lower::Session,
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
            symbols: Symbols::default(),
            next_chunk: 0,
            types: infer::Session::new(),
            lowering: lower::Session::new(),
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
        self.next_chunk += 1;
        let program = self.parse(source, chunk)?;
        let named_source = miette::NamedSource::new(format!("<repl:{chunk}>"), source.to_owned());
        // Each stage works on a copy of its environment, kept only once the
        // whole chunk has compiled.
        let mut types = self.types.clone();
        let checked = types
            .check(&program)
            .map_err(|(error, span)| report(error, span, &named_source))?;
        let mut lowering = self.lowering.clone();
        let module = lowering
            .lower(
                &program,
                &checked.types,
                &lower::Source {
                    file: format!("<repl:{chunk}>"),
                    text: source.to_owned(),
                },
                &format!("nassau_repl_{chunk}"),
            )
            .map_err(|(error, span)| report(error, span, &named_source))?;
        let entry = self
            .codegen
            .compile_jit_chunk(&mut self.module, &mut self.symbols, &module)
            .map_err(miette::Report::msg)?;
        self.types = types;
        self.lowering = lowering;
        let status = entry();
        for binding in &checked.bindings {
            let word = self
                .lowering
                .global(&binding.name)
                .and_then(|global| Codegen::global_address(&self.module, &self.symbols, global))
                // SAFETY: a global's cell holds one word, written by the chunk.
                .map(|address| unsafe { address.read() });
            let shown = word.map_or_else(|| "-".to_string(), |word| show(word, &binding.resolved));
            println!("val {} = {shown} : {}", binding.name, binding.ty);
        }
        if program.statements.is_empty() {
            println!("val it = {status} : int");
        }
        Ok(())
    }

    pub fn run(&mut self) -> miette::Result<()> {
        let stdin = io::stdin();
        let mut input = stdin.lock();
        let mut line = String::new();
        let mut chunk = String::new();
        loop {
            if chunk.is_empty() {
                print!("nassau> ");
                io::stdout()
                    .flush()
                    .map_err(|error| miette::miette!("{error}"))?;
            }
            line.clear();
            let bytes = input
                .read_line(&mut line)
                .map_err(|error| miette::miette!("{error}"))?;
            chunk.push_str(&line);
            // Like SML/NJ, a chunk runs at a semicolon ending a line, or at
            // the end of the input.
            let complete = bytes == 0 || chunk.trim_end().ends_with(';');
            if !complete {
                continue;
            }
            if !chunk.trim().is_empty()
                && let Err(error) = self.execute(&chunk)
            {
                eprintln!("{error:?}");
            }
            chunk.clear();
            if bytes == 0 {
                break;
            }
        }
        Ok(())
    }
}

fn report<E: std::error::Error + Send + Sync + 'static>(
    error: E,
    span: miette::SourceSpan,
    source: &miette::NamedSource<String>,
) -> miette::Report {
    miette::Report::new(SourceError::new(error, span)).with_source_code(source.clone())
}

/// A value as SML/NJ's top level prints it.
fn show(word: u64, ty: &Ty) -> String {
    let word = word as i64;
    // SAFETY (for every read below): the type says the word points to a heap
    // block of the matching shape.
    let field = |block: i64, index: usize| unsafe { *((block as *const i64).add(index + 1)) };
    match ty {
        Ty::Con {
            name,
            stamp: 0,
            args,
        } => match (name.as_str(), args.as_slice()) {
            ("int", _) => {
                let value = word >> 1;
                if value < 0 {
                    format!("~{}", -value)
                } else {
                    value.to_string()
                }
            }
            ("bool", _) => (word == value::TRUE).to_string(),
            ("unit", _) => "()".to_string(),
            ("char", _) => {
                let char = char::from_u32((word >> 1) as u32).unwrap_or('?');
                format!("#\"{}\"", escape(&char.to_string()))
            }
            ("string", _) => {
                // SAFETY: a string block holds its length and then its bytes.
                let text = unsafe {
                    let length = (*(word as *const i64) >> 8) as usize;
                    std::slice::from_raw_parts((word as *const u8).add(8), length)
                };
                format!("\"{}\"", escape(&String::from_utf8_lossy(text)))
            }
            ("real", _) => {
                let real = f64::from_bits(field(word, 0) as u64);
                let text = format!("{real:?}");
                text.replace('-', "~")
            }
            ("list", [element]) => {
                let mut items = Vec::new();
                let mut cell = word;
                while cell != value::NIL {
                    items.push(show(field(cell, 0) as u64, element));
                    cell = field(cell, 1);
                }
                format!("[{}]", items.join(","))
            }
            _ => "-".to_string(),
        },
        Ty::Arrow(..) => "fn".to_string(),
        Ty::Record(fields) if fields.is_empty() => "()".to_string(),
        Ty::Record(fields) => {
            let items: Vec<String> = fields
                .iter()
                .enumerate()
                .map(|(index, (_, ty))| show(field(word, index) as u64, ty))
                .collect();
            let tuple = fields
                .iter()
                .enumerate()
                .all(|(index, (label, _))| *label == (index + 1).to_string());
            if tuple {
                format!("({})", items.join(","))
            } else {
                let items: Vec<String> = fields
                    .iter()
                    .zip(items)
                    .map(|((label, _), item)| format!("{label}={item}"))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
        _ => "-".to_string(),
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
    .map_err(miette::Report::msg)?
    .run()
}

/// A string's characters as an SML string literal shows them.
fn escape(text: &str) -> String {
    let mut escaped = String::new();
    for char in text.chars() {
        match char {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            char if (char as u32) < 32 => {
                escaped.push_str(&format!("\\^{}", (char as u8 + 64) as char))
            }
            char => escaped.push(char),
        }
    }
    escaped
}
