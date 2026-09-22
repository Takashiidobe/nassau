mod codegen;
mod lexer;
mod parser;
mod repl;
mod span;

use std::fs;
use std::path::{Path, PathBuf};

use crate::codegen::{Codegen, OptLevel};
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
        .map_err(|error| miette::Report::new(error).with_source_code(named_source.clone()))?
        .parse()
        .map_err(|error| miette::Report::new(error).with_source_code(named_source))?;
    let output = if cli.asm {
        input.with_extension("S")
    } else {
        output_path(input).map_err(miette::Report::msg)?
    };
    Codegen::new(
        cli.opt_level,
        cli.debug_passes,
        cli.asm,
        cli.dump_ir,
        cli.dump_optimized_ir,
        cli.verify,
        cli.timings,
        cli.stats,
        cli.objdump,
    )
    .compile(&program, &output)
    .map_err(miette::Report::msg)?;
    Ok(output)
}

fn main() {
    let cli = Cli::parse();
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
