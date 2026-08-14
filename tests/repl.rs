mod common;

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

/// `(* XFAIL: reason *)` marks a fixture whose REPL output is known to differ.
fn has_xfail(source: &str) -> bool {
    source
        .lines()
        .any(|line| line.trim_start().starts_with("(* XFAIL"))
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

fn compare_repl(fixture: &Path, polyml: &str) {
    let source = fs::read_to_string(fixture).expect("read fixture");
    let input = repl_source(&source);
    let directory = fixture.parent().unwrap();
    let nassau = run_with_input(
        Command::new(env!("CARGO_BIN_EXE_nassau")).current_dir(directory),
        &input,
    )
    .expect("run Nassau REPL");
    let reference = Command::new(polyml)
        .args(["-q", "--script"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tools/polyml_oracle.sml"))
        .env("NASSAU_ORACLE_FILE", fixture)
        .current_dir(directory)
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|error| {
            panic!("run Poly/ML ({polyml}) for {}: {error}", fixture.display())
        });
    assert!(nassau.status.success(), "{}", fixture.display());
    let expected_exit = source
        .lines()
        .find_map(|line| {
            line.strip_prefix("(* ORACLE-EXIT: ")?
                .strip_suffix(" *)")?
                .parse::<i32>()
                .ok()
        })
        .unwrap_or(0);
    assert_eq!(
        reference.status.code(),
        Some(expected_exit),
        "{}: {}",
        fixture.display(),
        String::from_utf8_lossy(&reference.stderr)
    );
    assert!(
        !String::from_utf8_lossy(&reference.stderr).contains("Static Errors"),
        "{}: Poly/ML rejected the REPL input",
        fixture.display()
    );

    let transcript = String::from_utf8_lossy(&nassau.stdout)
        .replace("nassau> ", "")
        .trim_end()
        .to_owned()
        + "\n";
    let result = common::check_stream(fixture, &source, "CHECK-REPL", transcript.as_bytes(), true);
    match (has_xfail(&source), result) {
        (false, Ok(())) | (true, Err(_)) => {}
        (false, Err(error)) => panic!("{error}"),
        (true, Ok(())) => panic!(
            "{}: XFAIL fixture now passes; remove its XFAIL line",
            fixture.display()
        ),
    }
}

#[test]
fn repl_matches_polyml() {
    let explicit_polyml = std::env::var("POLYML").ok();
    let polyml = explicit_polyml.as_deref().unwrap_or("poly");
    if let Err(error) = Command::new(polyml).arg("--help").output() {
        if error.kind() == std::io::ErrorKind::NotFound && explicit_polyml.is_none() {
            eprintln!("skipping Poly/ML REPL comparison: set POLYML or install `polyml`");
            return;
        }
        panic!("could not start Poly/ML ({polyml}): {error}");
    }

    for fixture in fixtures() {
        compare_repl(&fixture, polyml);
    }
}
