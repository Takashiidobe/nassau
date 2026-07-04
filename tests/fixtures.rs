use libtest_mimic::{Arguments, Completion, Trial};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixtures() -> Vec<PathBuf> {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut paths = Vec::new();
    collect_fixtures(&directory, &mut paths);
    paths.sort();
    assert!(
        !paths.is_empty(),
        "tests/fixtures must contain at least one .sml fixture"
    );
    paths
}

fn collect_fixtures(directory: &Path, paths: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("read test fixtures directory") {
        let path = entry.expect("read fixture directory entry").path();
        if path.is_dir() {
            collect_fixtures(&path, paths);
        } else if path.extension().is_some_and(|extension| extension == "sml") {
            paths.push(path);
        }
    }
}

fn is_list_fixture(fixture: &Path) -> bool {
    fixture.parent().and_then(Path::file_name) == Some(std::ffi::OsStr::new("lists"))
}

fn fixture_id(fixture: &Path) -> String {
    fixture
        .strip_prefix(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures"))
        .expect("fixture is under tests/fixtures")
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("-")
}

fn compare_list_fixture(fixture: &Path) -> bool {
    let valid = fixture
        .file_stem()
        .expect("fixture has a file stem")
        .to_string_lossy()
        .starts_with("valid-");
    let directory = fixture.parent().expect("fixture has a parent directory");
    let nassau = Command::new(env!("CARGO_BIN_EXE_nassau"))
        .arg(fixture)
        .current_dir(directory)
        .output()
        .expect("run Nassau compiler");
    assert_eq!(
        nassau.status.success(),
        valid,
        "Nassau: {}",
        fixture.display()
    );

    if valid {
        let executable = fixture.with_extension("");
        let execution = Command::new(&executable)
            .current_dir(directory)
            .output()
            .expect("run Nassau output");
        let _ = fs::remove_file(&executable);
        assert!(execution.status.success(), "{}", fixture.display());
    }

    let smlnj = Command::new("smlnj")
        .arg(fixture)
        .current_dir(directory)
        .output()
        .expect("run SML/NJ");
    if valid {
        assert!(smlnj.status.success(), "SML/NJ: {}", fixture.display());
    }

    let oracle_executable = std::env::temp_dir().join(format!(
        "nassau-list-oracle-{}-{}",
        std::process::id(),
        fixture_id(fixture)
    ));
    let mlton = match Command::new("mlton")
        .args(["-output"])
        .arg(&oracle_executable)
        .arg(fixture)
        .current_dir(directory)
        .output()
    {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let _ = fs::remove_file(fixture.with_extension(""));
            eprintln!("skipping MLton oracle comparison: mlton is not installed");
            return false;
        }
        Err(error) => panic!("run MLton compiler: {error}"),
    };
    let _ = fs::remove_file(&oracle_executable);
    assert_eq!(
        mlton.status.success(),
        valid,
        "MLton disagreed on {}: {}",
        fixture.display(),
        String::from_utf8_lossy(&mlton.stderr)
    );
    true
}

fn compare_fixture(fixture: &Path) -> bool {
    if is_list_fixture(fixture) {
        return compare_list_fixture(fixture);
    }

    let source = fs::read_to_string(fixture).expect("read fixture");
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
        fixture_id(fixture)
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

    let _ = fs::remove_file(&nassau_executable);
    let _ = fs::remove_dir_all(&oracle_dir);
    true
}

fn main() {
    let arguments = Arguments::from_args();
    let fixtures_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let trials = fixtures()
        .into_iter()
        .map(|fixture| {
            let name = fixture
                .strip_prefix(&fixtures_root)
                .expect("fixture is under tests/fixtures")
                .display()
                .to_string();
            Trial::ignorable_test(name, move || {
                if compare_fixture(&fixture) {
                    Ok(Completion::Completed)
                } else {
                    Ok(Completion::ignored_with("mlton is not installed"))
                }
            })
        })
        .collect();
    libtest_mimic::run(&arguments, trials).exit();
}
