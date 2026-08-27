use std::fmt::Write;

use crate::core;
use crate::error::SourceError;
use crate::infer;
use crate::interpreter::{Interpreter, Signal, Value};
use crate::lower;
use crate::parser::{Fixity, Parser, Program};
use crate::printing::{Printer, ReplValue, uncaught};

pub enum Execution<V> {
    Returned(V),
    Raised(V),
    Exit(u8),
}

pub trait Backend {
    type Value: ReplValue;
    fn execute(&mut self, module: core::Module) -> miette::Result<Execution<Self::Value>>;
    fn global(&self, id: core::GlobalId) -> Option<Self::Value>;
    fn retain_globals(&mut self, roots: &[core::GlobalId]);
    fn reset(&mut self) -> miette::Result<()>;
    fn take_output(&mut self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Default)]
#[cfg_attr(feature = "web", derive(serde::Serialize))]
pub struct Response {
    pub output: Vec<u8>,
    pub diagnostics: String,
    pub exit: Option<u8>,
    pub clear: bool,
}

pub struct Session<B = Interpreter> {
    backend: B,
    next_chunk: usize,
    fixity: Option<Fixity>,
    types: infer::Session,
    lowering: lower::Session,
    lines: usize,
    exit: Option<u8>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new(Interpreter::default())
    }
}

impl<B: Backend> Session<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            next_chunk: 0,
            fixity: None,
            types: infer::Session::new(),
            lowering: lower::Session::new_repl(),
            lines: 0,
            exit: None,
        }
    }

    pub fn submit(&mut self, source: &str) -> Response {
        let mut response = Response {
            exit: self.exit,
            ..Response::default()
        };
        match source.trim().strip_suffix(";;").map(str::trim) {
            Some("clear") => {
                response.clear = true;
                response.output = b"Cleared\n".to_vec();
                return response;
            }
            Some("reset") => {
                if let Err(error) = self.backend.reset() {
                    writeln!(response.diagnostics, "{error:?}").unwrap();
                    return response;
                }
                self.next_chunk = 0;
                self.fixity = None;
                self.types = infer::Session::new();
                self.lowering = lower::Session::new_repl();
                self.lines = 0;
                self.exit = None;
                return Response {
                    clear: true,
                    output: b"Reset\n".to_vec(),
                    ..Response::default()
                };
            }
            _ => {}
        }
        if self.exit.is_some() {
            return response;
        }
        let first_line = self.lines + 1;
        self.lines += source.lines().count();
        if let Err(error) = self.execute(source, first_line, &mut response) {
            writeln!(response.diagnostics, "{error:?}").unwrap();
        }
        response
    }

    fn execute(
        &mut self,
        source: &str,
        first_line: usize,
        response: &mut Response,
    ) -> miette::Result<()> {
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
            if let Err(error) = self.execute_program(program, source, first_line, response) {
                writeln!(response.diagnostics, "{error:?}").unwrap();
            }
            if response.exit.is_some() {
                break;
            }
        }
        Ok(())
    }

    fn execute_program(
        &mut self,
        program: Program,
        source: &str,
        first_line: usize,
        response: &mut Response,
    ) -> miette::Result<()> {
        let chunk = self.next_chunk;
        self.next_chunk += 1;
        let named_source = miette::NamedSource::new(format!("<repl:{chunk}>"), source.to_owned());
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
                    file: "stdIn".into(),
                    text: source.to_owned(),
                    first_line,
                },
                &format!("nassau_repl_{chunk}"),
            )
            .map_err(|(error, span)| report(error, span, &named_source))?;
        let execution = self.backend.execute(module)?;
        response.output.extend(self.backend.take_output());
        let earlier_types = std::mem::replace(&mut self.types, types);
        let earlier_lowering = std::mem::replace(&mut self.lowering, lowering);
        let mut transcript = String::new();
        match execution {
            Execution::Raised(exception) => {
                self.types = earlier_types;
                self.lowering.forget_bindings(&earlier_lowering);
                writeln!(
                    transcript,
                    "\n{}",
                    exception.with_root(|| uncaught(exception.clone()))
                )
                .unwrap();
            }
            Execution::Exit(status) => {
                self.exit = Some(status);
                response.exit = self.exit;
            }
            Execution::Returned(status) => {
                let printer = Printer { types: &self.types };
                for index in 0..=checked.bindings.len() {
                    for (_, echo) in checked.echoes.iter().filter(|(at, _)| *at == index) {
                        writeln!(transcript, "{echo}").unwrap();
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
                    let value = self
                        .lowering
                        .global(&binding.name)
                        .and_then(|id| self.backend.global(id));
                    let shown = value.map_or_else(
                        || "-".into(),
                        |value| {
                            value.with_root(|| printer.show(value.clone(), &binding.resolved, 10))
                        },
                    );
                    writeln!(
                        transcript,
                        "val {} = {shown} : {}",
                        binding.name,
                        self.types.binding_type(&binding.resolved)
                    )
                    .unwrap();
                }
                if program.statements.is_empty() {
                    writeln!(transcript, "val it = {} : int", status.immediate() >> 1).unwrap();
                }
            }
        }
        response.output.extend(transcript.into_bytes());
        self.backend
            .retain_globals(&self.lowering.root_globals().into_iter().collect::<Vec<_>>());
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

impl Backend for Interpreter {
    type Value = Value;
    fn execute(&mut self, module: core::Module) -> miette::Result<Execution<Value>> {
        Ok(match self.run(module) {
            Ok(value) => Execution::Returned(value),
            Err(Signal::Raised(value)) => Execution::Raised(value),
            Err(Signal::Exit(status)) => Execution::Exit(status),
        })
    }
    fn global(&self, id: core::GlobalId) -> Option<Value> {
        self.global(id)
    }
    fn retain_globals(&mut self, roots: &[core::GlobalId]) {
        self.retain_globals(roots);
    }
    fn reset(&mut self) -> miette::Result<()> {
        *self = Self::default();
        Ok(())
    }
    fn take_output(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.output)
    }
}

impl ReplValue for Value {
    fn is_boxed(&self) -> bool {
        matches!(self, Value::Block(_))
    }
    fn immediate(&self) -> i64 {
        self.word()
    }
    fn field(&self, index: usize) -> Self {
        self.field(index)
    }
    fn length(&self) -> usize {
        self.object().fields.borrow().len()
    }
    fn bytes(&self) -> Vec<u8> {
        self.object().bytes.clone()
    }
    fn real(&self) -> f64 {
        self.object().real
    }
    fn builtin(&self, name: &str) -> bool {
        self.object().kind == crate::value::KIND_REF && self.field(0).bytes() == name.as_bytes()
    }
}
