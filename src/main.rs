mod codegen;
mod error;
mod lexer;
mod parser;
mod repl;
mod sema;
mod span;

use std::fs;
use std::path::{Path, PathBuf};

use crate::codegen::{Codegen, CodegenOptions, OptLevel};
use crate::error::SourceError;
use crate::lexer::Lexer;
use crate::parser::Parser as SmlParser;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "nassau",
    about = "Run the Nassau REPL or compile a source file"
)]
struct Cli {
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,
    #[arg(
        long,
        help = "Print IR before and after optimization and lowered instructions"
    )]
    debug_passes: bool,
    #[arg(long, help = "Write Cranelift's target instruction listing to FILE.S")]
    asm: bool,
    #[arg(long, help = "Print Cranelift IR before optimization")]
    dump_ir: bool,
    #[arg(long, help = "Print the lexical tokens for a source file")]
    dump_tokens: bool,
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

fn output_path(input: &Path) -> Result<PathBuf, String> {
    let stem = input
        .file_stem()
        .ok_or_else(|| "input path has no file name".to_string())?;
    Ok(input.with_file_name(stem))
}

fn run(cli: &Cli) -> miette::Result<PathBuf> {
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
    sema::Analyzer::new()
        .analyze_program(&program)
        .map_err(|(error, expr)| {
            miette::Report::new(SourceError::new(error, expr.source_span()))
                .with_source_code(named_source)
        })?;
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
    .compile(&program, &output)
    .map_err(miette::Report::msg)?;
    Ok(output)
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
    if cli.input.is_none() {
        if let Err(error) = repl::run(
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
        Ok(output) => println!("wrote {}", output.display()),
        Err(error) => {
            eprintln!("{error:?}");
            std::process::exit(1);
        }
    }
}
