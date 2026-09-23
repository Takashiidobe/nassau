use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn fixtures() -> Vec<PathBuf> {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/repl");
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

fn compare_repl(fixture: &Path, smlnj: &str) {
    let source = fs::read_to_string(fixture).expect("read fixture");
    let input = repl_source(&source);
    let directory = fixture.parent().unwrap();
    let nassau = run_with_input(
        Command::new(env!("CARGO_BIN_EXE_nassau")).current_dir(directory),
        &input,
    )
    .expect("run Nassau REPL");
    let reference = run_with_input(
        Command::new(smlnj).current_dir(directory),
        &smlnj_source(&source),
    )
    .unwrap_or_else(|error| panic!("run SML/NJ ({smlnj}) for {}: {error}", fixture.display()));
    assert!(nassau.status.success(), "{}", fixture.display());
    assert!(reference.status.success(), "{}", fixture.display());
}

#[test]
fn repl_matches_smlnj() {
    let explicit_smlnj = std::env::var("SMLNJ").ok();
    let smlnj = explicit_smlnj.as_deref().unwrap_or("smlnj");
    if let Err(error) = Command::new(smlnj).stdin(Stdio::null()).output() {
        if error.kind() == std::io::ErrorKind::NotFound && explicit_smlnj.is_none() {
            eprintln!("skipping SML/NJ REPL comparison: set SMLNJ or install `smlnj`");
            return;
        }
        panic!("could not start SML/NJ ({smlnj}): {error}");
    }

    for fixture in fixtures() {
        compare_repl(&fixture, smlnj);
    }
}
