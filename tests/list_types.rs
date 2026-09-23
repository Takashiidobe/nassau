use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixtures() -> Vec<PathBuf> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lists");
    let mut paths: Vec<_> = fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "sml"))
        .collect();
    paths.sort();
    paths
}

fn expect_valid(path: &Path) -> bool {
    path.file_stem()
        .unwrap()
        .to_string_lossy()
        .starts_with("valid-")
}

fn compare_fixture(source: &Path) {
    let valid = expect_valid(source);
    let nassau = Command::new(env!("CARGO_BIN_EXE_nassau"))
        .arg(source)
        .output()
        .unwrap();
    let nassau_stderr = String::from_utf8_lossy(&nassau.stderr);
    if valid {
        assert!(!nassau.status.success());
        assert!(
            nassau_stderr.contains("expected an integer or real expression"),
            "valid list fixture should pass parsing and semantic analysis before backend support: {nassau_stderr}"
        );
    } else {
        assert!(!nassau.status.success());
        assert!(
            nassau_stderr.contains("expected int, found bool"),
            "{}: {nassau_stderr}",
            source.display()
        );
    }

    let smlnj = Command::new("smlnj").arg(source).output().unwrap();
    let smlnj_stdout = String::from_utf8_lossy(&smlnj.stdout);
    if valid {
        assert!(smlnj.status.success());
        assert!(!smlnj_stdout.contains("Error:"), "{smlnj_stdout}");
    } else {
        assert!(smlnj_stdout.contains("Error:"), "{smlnj_stdout}");
    }

    let executable = std::env::temp_dir().join(format!(
        "nassau-list-oracle-{}-{}",
        std::process::id(),
        source.file_stem().unwrap().to_string_lossy()
    ));
    let mlton = Command::new("mlton")
        .args(["-output"])
        .arg(&executable)
        .arg(source)
        .output()
        .unwrap();
    let _ = fs::remove_file(executable);
    assert_eq!(
        mlton.status.success(),
        valid,
        "MLton disagreed on {}: {}",
        source.display(),
        String::from_utf8_lossy(&mlton.stderr)
    );
}

#[test]
fn list_fixtures_match_smlnj_and_mlton_frontends() {
    let fixtures = fixtures();
    assert!(!fixtures.is_empty(), "list fixtures directory is empty");
    for fixture in fixtures {
        compare_fixture(&fixture);
    }
}
