mod ast_dump;
mod codegen;
mod repl;
mod runtime;

use std::fs;
use std::path::{Path, PathBuf};

use crate::codegen::{Codegen, CodegenOptions, OptLevel};
use crate::error::SourceError;
use crate::lexer::Lexer;
use crate::parser::Parser as SmlParser;
use clap::Parser;
use nassau::prelude::Basis;
use nassau::{core, error, infer, interpreter, lexer, lower, matching, parser, scope, value};

#[derive(Debug, Parser)]
#[command(
    name = "nassau",
    about = "Run the Nassau REPL or compile a source file"
)]
struct Cli {
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,
    #[arg(long, conflicts_with_all = ["asm", "dump_ir", "dump_optimized_ir", "debug_passes", "objdump", "timings", "stats", "verify"], help = "Run a source file with the portable Rust interpreter")]
    interpret: bool,
    #[arg(
        long,
        help = "Print IR before and after optimization and lowered instructions"
    )]
    debug_passes: bool,
    #[arg(long, help = "Write Cranelift's target instruction listing to FILE.S")]
    asm: bool,
    #[arg(long, help = "Print the core IR the program lowers to")]
    dump_core: bool,
    #[arg(long, help = "Print Cranelift IR before optimization")]
    dump_ir: bool,
    #[arg(long, help = "Print the lexical tokens for a source file")]
    dump_tokens: bool,
    #[arg(long, help = "Print the parsed syntax tree for a source file")]
    dump_ast: bool,
    #[arg(long, help = "Print the inferred type of every top-level binding")]
    dump_types: bool,
    #[arg(long, help = "Print the inferred type of every expression and pattern")]
    dump_expr_types: bool,
    #[arg(long, help = "Print Cranelift IR after optimization")]
    dump_optimized_ir: bool,
    #[arg(long, help = "Verify IR before and after optimization")]
    verify: bool,
    #[arg(long, value_enum, default_value_t = OptLevel::Speed)]
    opt_level: OptLevel,
    #[arg(long, help = "Print compilation phase timings")]
    timings: bool,
    #[arg(long, help = "Print IR and generated-code statistics")]
    stats: bool,
    #[arg(
        long,
        conflicts_with = "asm",
        help = "Disassemble the emitted object with objdump"
    )]
    objdump: bool,
}

/// Reports scope errors and redundant rules as errors and non-exhaustive
/// matches as warnings.
fn check_matches(
    program: &crate::parser::Program,
    named_source: &miette::NamedSource<String>,
) -> miette::Result<()> {
    if let Some(diagnostic) = scope::check_program(program).into_iter().next() {
        return Err(
            miette::Report::new(SourceError::new(diagnostic.kind, diagnostic.span))
                .with_source_code(named_source.clone()),
        );
    }
    let diagnostics = matching::check_program(program);
    for diagnostic in &diagnostics {
        match diagnostic.kind {
            matching::MatchDiagnosticKind::NonExhaustive => eprintln!(
                "warning: match nonexhaustive at {}:{}",
                diagnostic.line, diagnostic.column
            ),
            matching::MatchDiagnosticKind::Redundant => {}
        }
    }
    if let Some(diagnostic) = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.kind == matching::MatchDiagnosticKind::Redundant)
    {
        return Err(miette::Report::new(SourceError::new(
            error::MatchErrorKind::Redundant,
            diagnostic.span,
        ))
        .with_source_code(named_source.clone()));
    }
    Ok(())
}

fn output_path(input: &Path) -> Result<PathBuf, String> {
    let stem = input
        .file_stem()
        .ok_or_else(|| "input path has no file name".to_string())?;
    Ok(input.with_file_name(stem))
}

/// Compiles the input file; with `--dump-core`, prints its IR instead.
fn run(cli: &Cli) -> miette::Result<Option<PathBuf>> {
    let input = cli
        .input
        .as_ref()
        .ok_or_else(|| miette::miette!("expected a source file"))?;
    let source = fs::read_to_string(input).map_err(|error| miette::miette!("{error}"))?;
    let named_source = miette::NamedSource::new(input.display().to_string(), source.clone());
    let program = SmlParser::from_source(&source, input.to_string_lossy().as_ref())
        .map_err(|error| {
            miette::Report::new(SourceError::from_span(error))
                .with_source_code(named_source.clone())
        })?
        .parse()
        .map_err(|error| {
            miette::Report::new(SourceError::from_span(error))
                .with_source_code(named_source.clone())
        })?;
    check_matches(&program, &named_source)?;
    let mut types = infer::Session::new();
    let basis = Basis::check(&mut types);
    let checked = types.check(&program).map_err(|(error, span)| {
        miette::Report::new(SourceError::new(error, span)).with_source_code(named_source.clone())
    })?;
    let file = input.file_name().map_or_else(
        || input.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    let source = lower::Source {
        file,
        text: source,
        first_line: 1,
    };
    let user = lower::Part {
        program: &program,
        types: &checked.types,
        source: &source,
    };
    let parts = if cli.dump_core {
        vec![user]
    } else {
        vec![basis.part(), user]
    };
    let module = lower::Session::new()
        .lower_parts(&parts, "main")
        .map_err(|(error, span)| {
            miette::Report::new(SourceError::new(error, span)).with_source_code(named_source)
        })?;
    if cli.dump_core {
        print!("{module}");
        return Ok(None);
    }
    if cli.interpret {
        use interpreter::{Interpreter, Signal, Value};
        use std::io::Write;

        let mut interpreter = Interpreter::default();
        let result = interpreter.run(module);
        std::io::stdout()
            .write_all(&interpreter.output)
            .map_err(miette::Report::msg)?;
        let status = match result {
            Ok(value) => (value.word() >> 1) as i32,
            Err(Signal::Exit(status)) => i32::from(status),
            Err(Signal::Raised(exception)) => {
                let name = exception.field(0).field(0);
                let argument = exception.field(1);
                let location = exception.field(2);
                let detail = match &argument {
                    Value::Word(word) => (word >> 1).to_string().into_bytes(),
                    Value::Block(object) if object.kind == value::KIND_STRING => {
                        [b"\"", object.bytes.as_slice(), b"\""].concat()
                    }
                    _ => b"<unknown>".to_vec(),
                };
                let separator: &[u8] = if matches!(argument, Value::Word(_)) {
                    b"\n raised at "
                } else {
                    b" raised at "
                };
                let diagnostic = [
                    b"/usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception ",
                    name.object().bytes.as_slice(),
                    b" with ",
                    &detail,
                    separator,
                    &location.object().bytes,
                    b"\n\n",
                ]
                .concat();
                std::io::stderr()
                    .write_all(&diagnostic)
                    .map_err(miette::Report::msg)?;
                1
            }
        };
        std::process::exit(status);
    }
    let output = if cli.asm {
        input.with_extension("S")
    } else {
        output_path(input).map_err(miette::Report::msg)?
    };
    Codegen::new(CodegenOptions {
        opt_level: cli.opt_level,
        debug_passes: cli.debug_passes,
        asm: cli.asm,
        dump_ir: cli.dump_ir,
        dump_optimized_ir: cli.dump_optimized_ir,
        verify: cli.verify,
        timings: cli.timings,
        stats: cli.stats,
        objdump: cli.objdump,
    })
    .compile(&module, &output)
    .map_err(miette::Report::msg)?;
    Ok(Some(output))
}

fn parse_file(input: &Path) -> miette::Result<crate::parser::Program> {
    let source = fs::read_to_string(input).map_err(|error| miette::miette!("{error}"))?;
    let named_source = miette::NamedSource::new(input.display().to_string(), source.clone());
    let program = SmlParser::from_source(&source, input.to_string_lossy().as_ref())
        .map_err(|error| {
            miette::Report::new(SourceError::from_span(error))
                .with_source_code(named_source.clone())
        })?
        .parse()
        .map_err(|error| {
            miette::Report::new(SourceError::from_span(error))
                .with_source_code(named_source.clone())
        })?;
    check_matches(&program, &named_source)?;
    Ok(program)
}

fn dump_types(input: &Path, every_node: bool) -> miette::Result<()> {
    let program = parse_file(input)?;
    let source = fs::read_to_string(input).map_err(|error| miette::miette!("{error}"))?;
    let named_source = miette::NamedSource::new(input.display().to_string(), source.clone());
    let mut types = infer::Session::new();
    Basis::check(&mut types);
    let checked = types.check(&program).map_err(|(error, span)| {
        miette::Report::new(SourceError::new(error, span)).with_source_code(named_source)
    })?;
    if !every_node {
        for binding in checked.bindings {
            println!("val {} : {}", binding.name, binding.ty);
        }
        return Ok(());
    }
    // Variables are named across the whole listing, so a variable shared by
    // two nodes has one name.
    let mut names = Vec::new();
    for node in checked.types.nodes() {
        let text = source[node.start.offset..node.end.offset].split_whitespace();
        println!(
            "{}:{}-{}:{} {} {} : {}",
            node.start.line,
            node.start.column,
            node.end.line,
            node.end.column,
            if node.pattern { "pat" } else { "exp" },
            text.collect::<Vec<_>>().join(" "),
            node.ty.show_named(&mut names)
        );
    }
    Ok(())
}

fn dump_tokens(input: &Path) -> miette::Result<()> {
    let source = fs::read_to_string(input).map_err(|error| miette::miette!("{error}"))?;
    let named_source = miette::NamedSource::new(input.display().to_string(), source.clone());
    let tokens = Lexer::new(&source, input).tokenize().map_err(|error| {
        miette::Report::new(SourceError::from_span(error)).with_source_code(named_source)
    })?;
    for token in tokens {
        println!("{:?}", token.value);
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if cli.dump_tokens {
        let Some(input) = cli.input.as_ref() else {
            eprintln!("--dump-tokens requires a source file");
            std::process::exit(2);
        };
        if let Err(error) = dump_tokens(input) {
            eprintln!("{error:?}");
            std::process::exit(1);
        }
        return;
    }
    if cli.dump_ast {
        let Some(input) = cli.input.as_ref() else {
            eprintln!("--dump-ast requires a source file");
            std::process::exit(2);
        };
        match parse_file(input) {
            Ok(program) => print!("{}", ast_dump::program(&program)),
            Err(error) => {
                eprintln!("{error:?}");
                std::process::exit(1);
            }
        }
        return;
    }
    if cli.dump_types || cli.dump_expr_types {
        let Some(input) = cli.input.as_ref() else {
            eprintln!("--dump-types requires a source file");
            std::process::exit(2);
        };
        if let Err(error) = dump_types(input, cli.dump_expr_types) {
            eprintln!("{error:?}");
            std::process::exit(1);
        }
        return;
    }
    if cli.input.is_none() {
        if let Err(error) = repl::run(
            cli.interpret,
            cli.opt_level,
            cli.debug_passes,
            cli.dump_ir,
            cli.dump_optimized_ir,
            cli.verify,
            cli.stats,
        ) {
            eprintln!("{error:?}");
            std::process::exit(1);
        }
        return;
    }
    match run(&cli) {
        Ok(Some(output)) => println!("wrote {}", output.display()),
        Ok(None) => {}
        Err(error) => {
            eprintln!("{error:?}");
            std::process::exit(1);
        }
    }
}
