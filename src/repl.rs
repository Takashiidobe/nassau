use std::io::{self, BufRead, IsTerminal, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::codegen::{Codegen, CodegenOptions, OptLevel, Symbols};
use crate::runtime;
use nassau::core;
use nassau::error::LexerErrorKind;
use nassau::lexer::{Lexer, TokenKind};
use nassau::printing::ReplValue;
use nassau::session::{Backend, Execution, Session};
use nassau::value;
use nu_ansi_term::{Color, Style};
use reedline::{
    ColumnarMenu, Completer, CompletionResult, DefaultPrompt, DefaultPromptSegment,
    FileBackedHistory, Highlighter, KeyCode, KeyModifiers, MenuBuilder, Reedline, ReedlineEvent,
    ReedlineMenu, Signal, Span as CompletionSpan, StyledText, Suggestion, ValidationResult,
    Validator, default_emacs_keybindings,
};

struct SmlValidator;

impl Validator for SmlValidator {
    fn validate(&self, line: &str) -> ValidationResult {
        if line.trim().is_empty() || line.trim_end().ends_with(';') {
            ValidationResult::Complete
        } else {
            ValidationResult::Incomplete
        }
    }
}

struct SmlCompleter {
    names: Arc<Mutex<Vec<String>>>,
}

impl Completer for SmlCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> CompletionResult {
        let before = &line[..pos.min(line.len())];
        let start = before
            .char_indices()
            .rev()
            .find(|(_, ch)| !completion_char(*ch))
            .map_or(0, |(offset, ch)| offset + ch.len_utf8());
        let prefix = &before[start..];
        if prefix.is_empty() {
            return CompletionResult::fresh(Vec::<Suggestion>::new());
        }
        let names = self.names.lock().unwrap_or_else(|error| error.into_inner());
        let suggestions = names
            .iter()
            .filter(|name| name.starts_with(prefix))
            .map(|name| Suggestion {
                value: name.clone(),
                span: CompletionSpan::new(start, pos),
                append_whitespace: false,
                ..Suggestion::default()
            })
            .collect::<Vec<_>>();
        CompletionResult::fresh(suggestions)
    }
}

fn completion_char(ch: char) -> bool {
    ch.is_alphanumeric() || matches!(ch, '_' | '\'' | '.')
}

struct SmlHighlighter;

impl Highlighter for SmlHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let (tokens, error_range, error_style) =
            match Lexer::new(line, Path::new("<repl>")).tokenize() {
                Ok(tokens) => (tokens, None, None),
                Err(error) => {
                    let offset = error.start.offset.min(line.len());
                    let prefix = &line[..offset];
                    let tokens = Lexer::new(prefix, Path::new("<repl>"))
                        .tokenize()
                        .unwrap_or_default();
                    let style = match error.value {
                        LexerErrorKind::UnterminatedComment => Style::new().fg(Color::DarkGray),
                        LexerErrorKind::UnterminatedString
                        | LexerErrorKind::UnterminatedStringEscape
                        | LexerErrorKind::UnsupportedStringEscape => Style::new().fg(Color::Green),
                        _ => Style::new().fg(Color::Red),
                    };
                    (tokens, Some(offset..line.len()), Some(style))
                }
            };
        let mut ranges = tokens
            .iter()
            .filter_map(|token| {
                let style = match &token.value {
                    TokenKind::Val
                    | TokenKind::If
                    | TokenKind::Then
                    | TokenKind::Else
                    | TokenKind::True
                    | TokenKind::False
                    | TokenKind::Reserved(_) => Style::new().bold().fg(Color::Purple),
                    TokenKind::String(_) | TokenKind::Character(_) => Style::new().fg(Color::Green),
                    TokenKind::Integer(_) | TokenKind::Word(_) | TokenKind::Real(_) => {
                        Style::new().fg(Color::Yellow)
                    }
                    TokenKind::TypeVariable(_) => Style::new().fg(Color::Red),
                    TokenKind::Identifier(_) | TokenKind::SymbolicIdentifier(_) => {
                        Style::new().fg(Color::Cyan)
                    }
                    TokenKind::Equals
                    | TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Star
                    | TokenKind::Slash
                    | TokenKind::Div
                    | TokenKind::Greater
                    | TokenKind::GreaterEqual
                    | TokenKind::Less
                    | TokenKind::LessEqual
                    | TokenKind::NotEquals
                    | TokenKind::Tilde
                    | TokenKind::Colon
                    | TokenKind::Bar
                    | TokenKind::Arrow
                    | TokenKind::FatArrow
                    | TokenKind::OpaqueAscription => Style::new().fg(Color::Magenta),
                    _ => return None,
                };
                Some((token.start.offset..token.end.offset, style))
            })
            .collect::<Vec<_>>();
        ranges.extend(
            comment_ranges(line)
                .into_iter()
                .map(|range| (range, Style::new().fg(Color::DarkGray))),
        );
        if let (Some(range), Some(style)) = (error_range, error_style) {
            ranges.push((range, style));
        }
        ranges.sort_by_key(|(range, _)| range.start);
        let mut styled = StyledText::new();
        let mut offset = 0;
        for (range, style) in ranges {
            let start = range.start.max(offset).min(line.len());
            let end = range.end.min(line.len());
            if start > offset {
                styled.push((Style::new(), line[offset..start].to_owned()));
            }
            if end > start {
                styled.push((style, line[start..end].to_owned()));
                offset = end;
            }
        }
        if offset < line.len() {
            styled.push((Style::new(), line[offset..].to_owned()));
        }
        styled
    }
}

fn comment_ranges(source: &str) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut offset = 0;
    while offset < source.len() {
        let rest = &source[offset..];
        if rest.starts_with("#\"") || rest.starts_with('"') {
            offset += if rest.starts_with("#\"") { 2 } else { 1 };
            while offset < source.len() {
                let ch = source[offset..].chars().next().unwrap();
                offset += ch.len_utf8();
                if ch == '\\' && offset < source.len() {
                    offset += source[offset..].chars().next().unwrap().len_utf8();
                } else if ch == '"' {
                    break;
                }
            }
        } else if rest.starts_with("(*") {
            let start = offset;
            offset += 2;
            let mut depth = 1;
            while offset < source.len() && depth > 0 {
                let rest = &source[offset..];
                if rest.starts_with("(*") {
                    depth += 1;
                    offset += 2;
                } else if rest.starts_with("*)") {
                    depth -= 1;
                    offset += 2;
                } else {
                    offset += rest.chars().next().unwrap().len_utf8();
                }
            }
            ranges.push(start..offset);
        } else {
            offset += rest.chars().next().unwrap().len_utf8();
        }
    }
    ranges
}

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
) -> miette::Result<i32> {
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

fn terminal<B: Backend>(mut session: Session<B>) -> miette::Result<i32> {
    if !io::stdin().is_terminal() {
        return terminal_pipe(session);
    }
    let data_dir = directories::BaseDirs::new()
        .ok_or_else(|| miette::miette!("could not locate the user's data directory"))?;
    let history_path = data_dir.data_dir().join("nassau").join("repl_history");
    let history =
        FileBackedHistory::with_file(10_000, history_path).map_err(miette::Report::msg)?;
    let completion_names = Arc::new(Mutex::new(session.completions("")));
    let completer = SmlCompleter {
        names: Arc::clone(&completion_names),
    };
    let menu = Box::new(ColumnarMenu::default().with_name("sml_completions"));
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Tab,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("sml_completions".to_owned()),
            ReedlineEvent::MenuNext,
        ]),
    );
    let mut editor = Reedline::create()
        .with_history(Box::new(history))
        .with_completer(Box::new(completer))
        .with_highlighter(Box::new(SmlHighlighter))
        .with_validator(Box::new(SmlValidator))
        .with_menu(ReedlineMenu::EngineCompleter(menu))
        .with_edit_mode(Box::new(reedline::Emacs::new(keybindings)));
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("nassau".to_owned()),
        DefaultPromptSegment::Empty,
    );
    loop {
        match editor.read_line(&prompt).map_err(miette::Report::msg)? {
            Signal::Success(source) if !source.trim().is_empty() => {
                if let Some(status) = submit(&mut session, &source)? {
                    return Ok(i32::from(status));
                }
                *completion_names
                    .lock()
                    .unwrap_or_else(|error| error.into_inner()) = session.completions("");
            }
            Signal::Success(_) => {}
            Signal::CtrlC => println!(),
            Signal::CtrlD => break,
            Signal::HostCommand(_) | Signal::ExternalBreak(_) => {}
            _ => {}
        }
    }
    Ok(0)
}

fn submit<B: Backend>(session: &mut Session<B>, source: &str) -> miette::Result<Option<u8>> {
    let response = session.submit(source);
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
    Ok(response.exit)
}

fn terminal_pipe<B: Backend>(mut session: Session<B>) -> miette::Result<i32> {
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
        if !chunk.trim().is_empty()
            && let Some(status) = submit(&mut session, &chunk)?
        {
            return Ok(i32::from(status));
        }
        chunk.clear();
        if bytes == 0 {
            break;
        }
    }
    Ok(0)
}
