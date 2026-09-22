use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cranelift_codegen::ir::{AbiParam, InstBuilder, types};
use cranelift_codegen::settings;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};

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

fn lower_and_link(value: i64, output: &Path) -> Result<(), String> {
    let flags = settings::Flags::new(settings::builder());
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
    module
        .define_function(function, &mut context)
        .map_err(|error| error.to_string())?;
    let object = module.finish().emit().map_err(|error| error.to_string())?;

    let object_path = env::temp_dir().join(format!("nassau-{}.o", std::process::id()));
    fs::write(&object_path, object).map_err(|error| error.to_string())?;
    let linker = env::var("NASSAU_CC").unwrap_or_else(|_| "cc".to_string());
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
    Ok(())
}

fn run(input: &Path) -> Result<PathBuf, String> {
    let source = fs::read_to_string(input).map_err(|error| error.to_string())?;
    let tokens = tokenize(&source)?;
    let value = parse(&tokens)?;
    if !(i32::MIN as i64..=i32::MAX as i64).contains(&value) {
        return Err("integer literal does not fit in i32".to_string());
    }
    let output = output_path(input)?;
    lower_and_link(value, &output)?;
    Ok(output)
}

fn main() {
    let input = match env::args_os().nth(1) {
        Some(input) => PathBuf::from(input),
        None => {
            eprintln!("usage: cargo run -- <file.ml>");
            std::process::exit(2);
        }
    };
    match run(&input) {
        Ok(output) => println!("wrote {}", output.display()),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
