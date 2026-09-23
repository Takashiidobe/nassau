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
    let directory = source.parent().unwrap();
    let nassau = Command::new(env!("CARGO_BIN_EXE_nassau"))
        .arg(source)
        .current_dir(directory)
        .output()
        .unwrap();
    if valid {
        assert!(nassau.status.success(), "{}", source.display());
        let executable = source.with_extension("");
        let execution = Command::new(&executable)
            .current_dir(directory)
            .output()
            .unwrap();
        let _ = fs::remove_file(&executable);
        assert!(execution.status.success());
    } else {
        assert!(!nassau.status.success());
    }

    let smlnj = Command::new("smlnj")
        .arg(source)
        .current_dir(directory)
        .output()
        .unwrap();
    if valid {
        assert!(smlnj.status.success(), "{}", source.display());
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
        .current_dir(directory)
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
