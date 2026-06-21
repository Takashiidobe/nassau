use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn fixtures() -> Vec<PathBuf> {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut paths: Vec<_> = fs::read_dir(directory)
        .expect("read test fixtures directory")
        .map(|entry| entry.expect("read fixture directory entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "sml"))
        .collect();
    paths.sort();
    paths
}

fn repl_source(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with("(*"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn smlnj_source(source: &str) -> String {
    repl_source(source)
}

fn run_with_input(command: &mut Command, input: &str) -> std::io::Result<Output> {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(input.as_bytes())?;
    child.wait_with_output()
}

fn normalized_stdout(output: &Output, prompt: &str, filter_val_echoes: bool) -> Vec<u8> {
    let stdout = String::from_utf8_lossy(&output.stdout).replace(prompt, "");
    stdout
        .lines()
        .filter(|line| !filter_val_echoes || !line.trim_start().starts_with("val it ="))
        .filter(|line| !line.starts_with("Standard ML of New Jersey"))
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes()
}

fn compare_repl(fixture: &Path, smlnj: &str) {
    let source = fs::read_to_string(fixture).expect("read fixture");
    let input = repl_source(&source);
    let nassau = run_with_input(&mut Command::new(env!("CARGO_BIN_EXE_nassau")), &input)
        .expect("run Nassau REPL");
    let reference = run_with_input(&mut Command::new(smlnj), &smlnj_source(&source))
        .unwrap_or_else(|error| panic!("run SML/NJ ({smlnj}) for {}: {error}", fixture.display()));

    assert_eq!(
        normalized_stdout(&nassau, "nassau> ", false),
        normalized_stdout(&reference, "- ", true),
        "REPL output differs for {}\nSML/NJ stderr: {}\nNassau stderr: {}",
        fixture.display(),
        String::from_utf8_lossy(&reference.stderr),
        String::from_utf8_lossy(&nassau.stderr)
    );
    assert_eq!(
        nassau.status.code(),
        reference.status.code(),
        "REPL exit status differs for {}\nSML/NJ stderr: {}\nNassau stderr: {}",
        fixture.display(),
        String::from_utf8_lossy(&reference.stderr),
        String::from_utf8_lossy(&nassau.stderr)
    );
}

#[test]
fn repl_matches_smlnj() {
    let explicit_smlnj = std::env::var("SMLNJ").ok();
    let smlnj = explicit_smlnj.as_deref().unwrap_or("sml");
    if let Err(error) = Command::new(smlnj).stdin(Stdio::null()).output() {
        if error.kind() == std::io::ErrorKind::NotFound && explicit_smlnj.is_none() {
            eprintln!("skipping SML/NJ REPL comparison: set SMLNJ or install `sml`");
            return;
        }
        panic!("could not start SML/NJ ({smlnj}): {error}");
    }

    for fixture in fixtures() {
        compare_repl(&fixture, smlnj);
    }
}

#[test]
fn repl_evaluates_real_division_and_addition() {
    let output = run_with_input(
        &mut Command::new(env!("CARGO_BIN_EXE_nassau")),
        "val half = 1.0 / 2.0\nval one = half + half\n",
    )
    .expect("run Nassau REPL");

    assert!(output.status.success());
    assert_eq!(
        normalized_stdout(&output, "nassau> ", false),
        b"val half = 0.5 : real\nval one = 1.0 : real"
    );
}
