use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn fixture_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/ret-42.ml")
}

fn expected_exit_code(source: &str) -> i32 {
    source
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("(* exit_code: ")
                .and_then(|line| line.strip_suffix(" *)"))
        })
        .expect("fixture must declare an expected exit code")
        .parse()
        .expect("expected exit code must be an integer")
}

#[test]
fn fixture_exit_code_matches_comment() {
    let fixture = fixture_path();
    let source = fs::read_to_string(&fixture).expect("read fixture");
    let expected = expected_exit_code(&source);
    let compiler_status = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .arg(&fixture)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("run compiler");
    assert!(
        compiler_status.success(),
        "compiler failed: {compiler_status}"
    );

    let executable = fixture.with_file_name("ret-42");
    let executable_status = Command::new(&executable)
        .status()
        .expect("run generated executable");
    let _ = fs::remove_file(&executable);
    assert_eq!(executable_status.code(), Some(expected));
}
