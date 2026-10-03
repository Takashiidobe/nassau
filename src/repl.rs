use std::io::{self, BufRead, Write};

use crate::codegen::{Codegen, CodegenOptions, OptLevel, Symbols};
use crate::error::{CodegenError, SourceError};
use crate::infer::{self, Ty};
use crate::lower;
use crate::parser::{Fixity, Parser, Program};
use crate::runtime;
use crate::value;

pub struct Repl {
    codegen: Codegen,
    module: cranelift_jit::JITModule,
    symbols: Symbols,
    next_chunk: usize,
    fixity: Option<Fixity>,
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
        runtime::enter_repl();
        Ok(Self {
            codegen,
            module,
            symbols: Symbols::default(),
            next_chunk: 0,
            fixity: None,
            types: infer::Session::new(),
            lowering: lower::Session::new_repl(),
        })
    }

    fn execute(&mut self, source: &str, first_line: usize) -> miette::Result<()> {
        let filename = format!("<repl:{}>", self.next_chunk);
        let named_source = miette::NamedSource::new(filename.clone(), source.to_owned());
        let mut parser = Parser::from_repl_source(source, &filename)
            .map_err(|error| {
                miette::Report::new(SourceError::from_span(error))
                    .with_source_code(named_source.clone())
            })?
            .with_fixity(self.fixity.as_ref());
        while let Some((program, fixity)) = parser.repl_chunk().map_err(|error| {
            miette::Report::new(SourceError::from_span(error))
                .with_source_code(named_source.clone())
        })? {
            self.fixity = Some(fixity);
            if let Err(error) = self.execute_program(program, source, first_line) {
                eprintln!("{error:?}");
            }
        }
        Ok(())
    }

    fn execute_program(
        &mut self,
        program: Program,
        source: &str,
        first_line: usize,
    ) -> miette::Result<()> {
        let chunk = self.next_chunk;
        self.next_chunk += 1;
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
                    file: "stdIn".to_string(),
                    text: source.to_owned(),
                    first_line,
                },
                &format!("nassau_repl_{chunk}"),
            )
            .map_err(|(error, span)| report(error, span, &named_source))?;
        let entry = self
            .codegen
            .compile_jit_chunk(&mut self.module, &mut self.symbols, &module)
            .map_err(miette::Report::msg)?;
        let earlier_types = std::mem::replace(&mut self.types, types);
        let earlier_lowering = std::mem::replace(&mut self.lowering, lowering);
        let status = entry();
        if let Some(exception) = runtime::take_uncaught() {
            // A chunk that raises binds nothing.
            self.types = earlier_types;
            self.lowering.forget_bindings(&earlier_lowering);
            let shown = runtime::with_root(exception, || uncaught(exception));
            println!("\n{shown}");
            self.publish_global_roots();
            return Ok(());
        }
        let printer = Printer { types: &self.types };
        for index in 0..=checked.bindings.len() {
            for (_, echo) in checked.echoes.iter().filter(|(at, _)| *at == index) {
                println!("{echo}");
            }
            let Some(binding) = checked.bindings.get(index) else {
                continue;
            };
            if checked.bindings[index + 1..]
                .iter()
                .any(|later| later.name == binding.name)
            {
                continue;
            }
            let word = self
                .lowering
                .global(&binding.name)
                .and_then(|global| Codegen::global_address(&self.module, &self.symbols, global))
                // SAFETY: a global's cell holds one word, written by the chunk.
                .map(|address| unsafe { address.read() });
            let shown = word.map_or_else(
                || "-".to_string(),
                |word| {
                    runtime::with_root(word as i64, || printer.show(word, &binding.resolved, 10))
                },
            );
            println!(
                "val {} = {shown} : {}",
                binding.name,
                self.types.binding_type(&binding.resolved)
            );
        }
        if program.statements.is_empty() {
            println!("val it = {status} : int");
        }
        self.publish_global_roots();
        Ok(())
    }

    fn publish_global_roots(&self) {
        let addresses: Vec<_> = self
            .lowering
            .root_globals()
            .into_iter()
            .filter_map(|global| Codegen::global_address(&self.module, &self.symbols, global))
            .map(|address| address as usize)
            .collect();
        runtime::replace_global_roots(&addresses);
    }

    pub fn run(&mut self) -> miette::Result<()> {
        let stdin = io::stdin();
        let mut input = stdin.lock();
        let mut line = String::new();
        let mut chunk = String::new();
        // Lines are numbered across the whole input, as SML/NJ's `stdIn`.
        let mut lines = 0;
        let mut first_line = 1;
        loop {
            if chunk.is_empty() {
                first_line = lines + 1;
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
            lines += 1;
            // Like SML/NJ, a chunk runs at a semicolon ending a line, or at
            // the end of the input.
            let complete = bytes == 0 || chunk.trim_end().ends_with(';');
            if !complete {
                continue;
            }
            if !chunk.trim().is_empty()
                && let Err(error) = self.execute(&chunk, first_line)
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
struct Printer<'a> {
    types: &'a infer::Session,
}

impl Printer<'_> {
    fn show(&self, word: u64, ty: &Ty, depth: usize) -> String {
        if depth == 0 {
            return "#".to_string();
        }
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
                ("word", _) => format!("0wx{:X}", word >> 1),
                ("order", _) => ["LESS", "EQUAL", "GREATER"][(word >> 1) as usize].to_string(),
                ("option", [element]) if word == value::tagged(0) => "NONE".to_string(),
                ("option", [element]) => {
                    format!(
                        "SOME {}",
                        argument(self.show(field(word, 0) as u64, element, depth - 1))
                    )
                }
                ("ref", [element]) => {
                    format!(
                        "ref {}",
                        argument(self.show(field(word, 0) as u64, element, depth - 1))
                    )
                }
                ("exn", _) => {
                    let identity = field(word, 0);
                    let name = self.show(
                        field(identity, 0) as u64,
                        &Ty::Con {
                            name: "string".into(),
                            stamp: 0,
                            args: vec![],
                        },
                        depth,
                    );
                    let name = name.trim_matches('"');
                    let described = if unsafe { *(identity as *const i64) >> 8 } > 1 {
                        let descriptor = field(identity, 1);
                        (descriptor != value::NIL).then(|| descriptor_type(descriptor))
                    } else {
                        (identity
                            == runtime::builtin_exception(
                                value::builtin_exception("Fail").expect("basis Fail"),
                            ))
                        .then(|| Ty::Con {
                            name: "string".into(),
                            stamp: 0,
                            args: vec![],
                        })
                    };
                    match described {
                        Some(argument_ty) => format!(
                            "{name} {}",
                            argument(self.show(field(word, 1) as u64, &argument_ty, depth - 1))
                        ),
                        None => name.to_string(),
                    }
                }
                ("unit", _) => "()".to_string(),
                ("char", _) => {
                    format!("#\"{}\"", escape(&[(word >> 1) as u8]))
                }
                ("string", _) => {
                    // SAFETY: a string block holds its length and then its bytes.
                    let text = unsafe {
                        let length = (*(word as *const i64) >> 8) as usize;
                        std::slice::from_raw_parts((word as *const u8).add(8), length)
                    };
                    format!("\"{}\"", escape(text))
                }
                ("real", _) => {
                    let real = f64::from_bits(field(word, 0) as u64);
                    let text = format!("{real:?}");
                    text.replace('-', "~")
                }
                ("list", [element]) => {
                    let mut items = Vec::new();
                    let mut cell = word;
                    while cell != value::NIL && items.len() < 20 {
                        items.push(self.show(field(cell, 0) as u64, element, depth - 1));
                        cell = field(cell, 1);
                    }
                    if cell != value::NIL {
                        items.push("...".to_string());
                    }
                    format!("[{}]", items.join(","))
                }
                _ => "-".to_string(),
            },
            Ty::Con { .. } => {
                let Some(constructors) = self.types.constructors(ty) else {
                    return "-".to_string();
                };
                let carries = word & 1 == 0;
                let carriers = constructors
                    .iter()
                    .filter(|(_, argument)| argument.is_some())
                    .count();
                let index = if carries {
                    if carriers > 1 { field(word, 0) >> 1 } else { 0 }
                } else {
                    word >> 1
                };
                let Some((name, payload)) = constructors
                    .iter()
                    .filter(|(_, argument)| argument.is_some() == carries)
                    .nth(index as usize)
                else {
                    return "-".to_string();
                };
                match payload {
                    Some(payload) => format!(
                        "{name} {}",
                        argument(self.show(
                            field(word, usize::from(carriers > 1)) as u64,
                            payload,
                            depth - 1
                        ))
                    ),
                    None => name.clone(),
                }
            }
            Ty::Arrow(..) => "fn".to_string(),
            Ty::Record(fields) if fields.is_empty() => "()".to_string(),
            Ty::Record(fields) => {
                let items: Vec<String> = fields
                    .iter()
                    .enumerate()
                    .map(|(index, (_, ty))| self.show(field(word, index) as u64, ty, depth - 1))
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
}

/// SML/NJ's report of an exception that escapes a top-level declaration.
fn uncaught(exception: i64) -> String {
    // SAFETY: an exception value is a record of its identity, its argument
    // and the string naming where it was raised; the identity is a
    // reference cell holding the exception's name.
    let field = |block: i64, index: usize| unsafe { *((block as *const i64).add(index + 1)) };
    let text = |string: i64| unsafe {
        let length = (*(string as *const i64) >> 8) as usize;
        String::from_utf8_lossy(std::slice::from_raw_parts(
            (string as *const u8).add(8),
            length,
        ))
        .into_owned()
    };
    let identity = field(exception, 0);
    let name = text(field(identity, 0));
    let builtin = |known: &str| {
        value::builtin_exception(known)
            .is_some_and(|index| runtime::builtin_exception(index) == identity)
    };
    // The basis's exceptions describe themselves.
    let detail = if builtin("Fail") {
        format!(" [Fail: {}]", text(field(exception, 1)))
    } else {
        [
            ("Div", "divide by zero"),
            ("Overflow", "overflow"),
            ("Match", "nonexhaustive match failure"),
            ("Bind", "nonexhaustive binding failure"),
            ("Subscript", "subscript out of bounds"),
        ]
        .iter()
        .find(|(known, _)| builtin(known))
        .map_or_else(String::new, |(_, message)| format!(" [{message}]"))
    };
    format!(
        "uncaught exception {name}{detail}\n  raised at: {}",
        text(field(exception, 2))
    )
}

/// A constructor's argument as SML/NJ shows it: parenthesised when it is
/// itself a constructor application.
fn descriptor_type(descriptor: i64) -> Ty {
    let field = |block: i64, index: usize| unsafe { *((block as *const i64).add(index + 1)) };
    let fields = |block: i64| unsafe {
        std::slice::from_raw_parts(
            (block as *const i64).add(1),
            (*(block as *const i64) >> 8) as usize,
        )
    };
    let text = |block: i64| unsafe {
        String::from_utf8_lossy(std::slice::from_raw_parts(
            (block as *const u8).add(8),
            (*(block as *const i64) >> 8) as usize,
        ))
        .into_owned()
    };
    match field(descriptor, 0) >> 1 {
        0 => Ty::Con {
            name: text(field(descriptor, 1)),
            stamp: (field(descriptor, 2) >> 1) as usize,
            args: fields(field(descriptor, 3))
                .iter()
                .map(|&word| descriptor_type(word))
                .collect(),
        },
        1 => Ty::Record(
            fields(field(descriptor, 1))
                .iter()
                .map(|&word| (text(field(word, 0)), descriptor_type(field(word, 1))))
                .collect(),
        ),
        2 => Ty::Arrow(Box::new(Ty::Record(vec![])), Box::new(Ty::Record(vec![]))),
        _ => Ty::Var {
            id: 0,
            equality: false,
        },
    }
}

fn argument(shown: String) -> String {
    if shown.contains(' ') && !shown.starts_with(['(', '[', '{', '"', '#']) {
        format!("({shown})")
    } else {
        shown
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
fn escape(text: &[u8]) -> String {
    let mut escaped = String::new();
    for &byte in text {
        match byte {
            b'"' => escaped.push_str("\\\""),
            b'\\' => escaped.push_str("\\\\"),
            b'\n' => escaped.push_str("\\n"),
            b'\t' => escaped.push_str("\\t"),
            b'\r' => escaped.push_str("\\r"),
            8 => escaped.push_str("\\b"),
            12 => escaped.push_str("\\f"),
            0..32 => escaped.push_str(&format!("\\^{}", (byte + 64) as char)),
            127..=255 => escaped.push_str(&format!("\\{byte:03}")),
            byte => escaped.push(byte as char),
        }
    }
    escaped
}
