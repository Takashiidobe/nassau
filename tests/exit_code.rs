use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixtures() -> Vec<PathBuf> {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut paths: Vec<_> = fs::read_dir(directory)
        .expect("read test fixtures directory")
        .map(|entry| entry.expect("read fixture directory entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "sml"))
        .collect();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "tests/fixtures must contain at least one .sml fixture"
    );
    paths
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

fn compare_fixture(fixture: &Path) -> bool {
    let source = fs::read_to_string(fixture).expect("read fixture");
    let expected = expected_exit_code(&source);
    let compiler_status = Command::new(env!("CARGO_BIN_EXE_nassau"))
        .arg(fixture)
        .output()
        .expect("run Nassau compiler");
    assert!(
        compiler_status.status.success(),
        "Nassau compiler failed for {}: {}",
        fixture.display(),
        String::from_utf8_lossy(&compiler_status.stderr)
    );
    let stem = fixture.file_stem().expect("fixture has a file stem");
    let nassau_executable = fixture.with_file_name(stem);
    let nassau_output = Command::new(&nassau_executable)
        .output()
        .expect("run Nassau output");
    let oracle_dir = std::env::temp_dir().join(format!(
        "nassau-mlton-{}-{}",
        std::process::id(),
        stem.to_string_lossy()
    ));
    fs::create_dir_all(&oracle_dir).expect("create MLton output directory");
    let oracle_source = oracle_dir.join("input.sml");
    fs::write(&oracle_source, &source).expect("write MLton source");
    let oracle_executable = oracle_dir.join("oracle");
    let mlton = match Command::new("mlton")
        .args(["-output"])
        .arg(&oracle_executable)
        .arg(&oracle_source)
        .output()
    {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let _ = fs::remove_file(&nassau_executable);
            let _ = fs::remove_dir_all(&oracle_dir);
            eprintln!("skipping MLton oracle comparison: mlton is not installed");
            return false;
        }
        Err(error) => panic!("run MLton compiler: {error}"),
    };
    assert!(
        mlton.status.success(),
        "MLton compilation failed for {}: {}",
        fixture.display(),
        String::from_utf8_lossy(&mlton.stderr)
    );
    let mlton_output = Command::new(&oracle_executable)
        .output()
        .expect("run MLton output");

    assert_eq!(
        nassau_output.stdout,
        mlton_output.stdout,
        "stdout differs for {}",
        fixture.display()
    );
    assert_eq!(
        nassau_output.status.code(),
        mlton_output.status.code(),
        "exit status differs for {}",
        fixture.display()
    );
    assert_eq!(
        nassau_output.status.code(),
        Some(expected),
        "exit status differs from fixture expectation for {}",
        fixture.display()
    );

    let _ = fs::remove_file(&nassau_executable);
    let _ = fs::remove_dir_all(&oracle_dir);
    true
}

#[test]
fn all_fixtures_match_mlton() {
    for fixture in fixtures() {
        if !compare_fixture(&fixture) {
            return;
        }
    }
}
