use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use clap::{Parser, ValueEnum};
use cranelift_codegen::ir::{AbiParam, InstBuilder, types};
use cranelift_codegen::settings::Configurable;
use cranelift_codegen::{self, settings};
use cranelift_control::ControlPlane;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OptLevel {
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

#[derive(Debug, Parser)]
#[command(name = "nassau", about = "Compile a Nassau source file")]
struct Cli {
    #[arg(value_name = "FILE")]
    input: PathBuf,
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

#[derive(Debug)]
enum Token {
    Integer(i64),
}

fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut source_without_comments = String::new();
    let mut characters = source.chars().peekable();
    let mut comment_depth = 0;
    while let Some(character) = characters.next() {
        if character == '(' && characters.peek() == Some(&'*') {
            characters.next();
            comment_depth += 1;
        } else if character == '*' && characters.peek() == Some(&')') && comment_depth > 0 {
            characters.next();
            comment_depth -= 1;
        } else if comment_depth == 0 {
            source_without_comments.push(character);
        }
    }
    if comment_depth != 0 {
        return Err("unterminated comment".to_string());
    }
    let value = source_without_comments
        .trim()
        .parse::<i64>()
        .map_err(|_| "expected one integer literal".to_string())?;
    Ok(vec![Token::Integer(value)])
}

fn parse(tokens: &[Token]) -> Result<i64, String> {
    match tokens {
        [Token::Integer(value)] => Ok(*value),
        _ => Err("expected one integer literal".to_string()),
    }
}

fn output_path(input: &Path) -> Result<PathBuf, String> {
    let stem = input
        .file_stem()
        .ok_or_else(|| "input path has no file name".to_string())?;
    Ok(input.with_file_name(stem))
}

fn lower_and_link(value: i64, output: &Path, cli: &Cli) -> Result<(), String> {
    let total_start = Instant::now();
    let mut flag_builder = settings::builder();
    flag_builder
        .set("opt_level", cli.opt_level.as_cranelift())
        .map_err(|error| error.to_string())?;
    let flags = settings::Flags::new(flag_builder);
    let isa = cranelift_native::builder()
        .map_err(|error| error.to_string())?
        .finish(flags)
        .map_err(|error| error.to_string())?;
    let frontend_config = isa.frontend_config();
    let object_builder = ObjectBuilder::new(isa, "nassau", default_libcall_names())
        .map_err(|error| error.to_string())?;
    let mut module = ObjectModule::new(object_builder);
    let mut signature = module.make_signature();
    signature.returns.push(AbiParam::new(types::I32));
    let function = module
        .declare_function("main", Linkage::Export, &signature)
        .map_err(|error| error.to_string())?;

    let mut context = module.make_context();
    context.func.signature = signature;
    let mut builder_context = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
    let block = builder.create_block();
    builder.switch_to_block(block);
    builder.seal_block(block);
    let result = builder.ins().iconst(types::I32, value);
    builder.ins().return_(&[result]);
    builder.finalize(frontend_config);
    let frontend_time = total_start.elapsed();
    let dump_ir = cli.debug_passes || cli.dump_ir;
    let dump_optimized_ir = cli.debug_passes || cli.dump_optimized_ir;
    let needs_manual_optimization = dump_optimized_ir || cli.verify || cli.stats;
    if cli.verify {
        cranelift_codegen::verify_function(&context.func, module.isa())
            .map_err(|error| error.to_string())?;
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
            .optimize(module.isa(), &mut ControlPlane::default())
            .map_err(|error| error.to_string())?;
    }
    let optimization_time = optimization_start.elapsed();
    if cli.verify && needs_manual_optimization {
        cranelift_codegen::verify_function(&context.func, module.isa())
            .map_err(|error| error.to_string())?;
    }
    if dump_optimized_ir {
        println!(
            "== Cranelift IR after optimization ==\n{}",
            context.func.display()
        );
    }
    context.set_disasm(cli.debug_passes || cli.asm);
    let codegen_start = Instant::now();
    module
        .define_function(function, &mut context)
        .map_err(|error| error.to_string())?;
    let codegen_time = codegen_start.elapsed();
    if cli.stats {
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
    if cli.debug_passes || cli.asm {
        let disassembly = context
            .compiled_code()
            .and_then(|compiled| compiled.vcode.as_deref())
            .ok_or_else(|| "target does not provide a textual assembly listing".to_string())?;
        if cli.debug_passes {
            println!("== Cranelift machine instructions ==\n{disassembly}");
        }
        if cli.asm {
            fs::write(output, disassembly).map_err(|error| error.to_string())?;
            if cli.timings {
                println!(
                    "== Timings ==\nfrontend: {:?}\noptimization: {:?}\ncodegen: {:?}\ntotal: {:?}",
                    frontend_time,
                    optimization_time,
                    codegen_time,
                    total_start.elapsed()
                );
            }
            return Ok(());
        }
    }
    let object = module.finish().emit().map_err(|error| error.to_string())?;

    let object_path = env::temp_dir().join(format!("nassau-{}.o", std::process::id()));
    fs::write(&object_path, object).map_err(|error| error.to_string())?;
    if cli.objdump {
        let objdump = Command::new("objdump")
            .args(["-drwC"])
            .arg(&object_path)
            .output()
            .map_err(|error| format!("failed to invoke objdump: {error}"))?;
        if !objdump.status.success() {
            let _ = fs::remove_file(&object_path);
            return Err(String::from_utf8_lossy(&objdump.stderr).trim().to_string());
        }
        println!(
            "== Object disassembly ==\n{}",
            String::from_utf8_lossy(&objdump.stdout)
        );
    }
    let linker = env::var("NASSAU_CC").unwrap_or_else(|_| "cc".to_string());
    let link_start = Instant::now();
    let link_result = Command::new(&linker)
        .args(["-o"])
        .arg(output)
        .arg(&object_path)
        .output()
        .map_err(|error| format!("failed to invoke {linker}: {error}"))?;
    let _ = fs::remove_file(&object_path);
    if !link_result.status.success() {
        return Err(String::from_utf8_lossy(&link_result.stderr)
            .trim()
            .to_string());
    }
    if cli.timings {
        println!(
            "== Timings ==\nfrontend: {:?}\noptimization: {:?}\ncodegen: {:?}\nlink: {:?}\ntotal: {:?}",
            frontend_time,
            optimization_time,
            codegen_time,
            link_start.elapsed(),
            total_start.elapsed()
        );
    }
    Ok(())
}

fn run(cli: &Cli) -> Result<PathBuf, String> {
    let input = &cli.input;
    let source = fs::read_to_string(input).map_err(|error| error.to_string())?;
    let tokens = tokenize(&source)?;
    let value = parse(&tokens)?;
    if !(i32::MIN as i64..=i32::MAX as i64).contains(&value) {
        return Err("integer literal does not fit in i32".to_string());
    }
    let output = if cli.asm {
        input.with_extension("S")
    } else {
        output_path(input)?
    };
    lower_and_link(value, &output, cli)?;
    Ok(output)
}

fn main() {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(output) => println!("wrote {}", output.display()),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
