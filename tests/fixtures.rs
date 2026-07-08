mod common;

use libtest_mimic::{Arguments, Completion, Trial};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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

fn expected_valid(fixture: &Path) -> bool {
    match fixture.parent().and_then(Path::file_name) {
        Some(name) if name == "lists" => fixture
            .file_stem()
            .expect("fixture has a file stem")
            .to_string_lossy()
            .starts_with("valid-"),
        Some(name) if name == "lexer" => fixture
            .file_stem()
            .expect("fixture has a file stem")
            .to_string_lossy()
            .starts_with("valid-"),
        _ => !fixture
            .file_stem()
            .expect("fixture has a file stem")
            .to_string_lossy()
            .starts_with("invalid-"),
    }
}

fn is_lexer_fixture(fixture: &Path) -> bool {
    fixture.parent().and_then(Path::file_name) == Some(std::ffi::OsStr::new("lexer"))
}

fn compare_mlton(fixture: &Path, mlton: &str, valid: bool) {
    if !is_lexer_fixture(fixture) {
        return;
    }
    let executable = std::env::temp_dir().join(format!(
        "nassau-mlton-{}-{}",
        std::process::id(),
        fixture
            .file_stem()
            .expect("fixture has a file stem")
            .to_string_lossy()
    ));
    let _ = fs::remove_file(&executable);
    let result = Command::new(mlton)
        .arg("-output")
        .arg(&executable)
        .arg(fixture)
        .current_dir(fixture.parent().expect("fixture has a parent directory"))
        .output()
        .unwrap_or_else(|error| panic!("run MLton ({mlton}): {error}"));
    assert_eq!(
        result.status.success(),
        valid,
        "MLton: {}: {}",
        fixture.display(),
        String::from_utf8_lossy(&result.stderr)
    );
    let _ = fs::remove_file(executable);
}

fn smlnj_program_output(output: &[u8], fixture: &Path) -> Vec<u8> {
    let output = String::from_utf8_lossy(output);
    if let Some(start) = output.find("[autoloading done]\n") {
        return output[start + "[autoloading done]\n".len()..]
            .as_bytes()
            .to_vec();
    }
    let opening = format!("[opening {}]\n", fixture.display());
    let start = output.find(&opening).unwrap_or_else(|| {
        panic!(
            "SML/NJ output did not contain {opening:?} for {}",
            fixture.display()
        )
    });
    output[start + opening.len()..].as_bytes().to_vec()
}

fn check_program(fixture: &Path, source: &str, output: &Output) {
    let code = output.status.code();
    if source.contains("(* CHECK-EXIT") {
        let code = code.map_or_else(|| "signal".to_owned(), |code| code.to_string());
        common::check_stream(
            fixture,
            source,
            "CHECK-EXIT",
            format!("{code}\n").as_bytes(),
            true,
        )
        .unwrap_or_else(|error| panic!("{error}"));
    } else {
        assert_eq!(
            code,
            Some(0),
            "no CHECK-EXIT, expected success: {}",
            fixture.display()
        );
    }
    common::check_stream(fixture, source, "CHECK-STDOUT", &output.stdout, true)
        .unwrap_or_else(|error| panic!("{error}"));
    common::check_stream(fixture, source, "CHECK-STDERR", &output.stderr, true)
        .unwrap_or_else(|error| panic!("{error}"));
}

fn compare_fixture(fixture: &Path, smlnj: &str) {
    let valid = expected_valid(fixture);
    let directory = fixture.parent().expect("fixture has a parent directory");
    let mut nassau_command = Command::new(env!("CARGO_BIN_EXE_nassau"));
    if is_lexer_fixture(fixture) {
        nassau_command.arg("--dump-tokens");
    }
    let nassau = nassau_command
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
    let source = fs::read_to_string(fixture).expect("read fixture");
    if !valid {
        common::check_stream(fixture, &source, "CHECK-ERR", &nassau.stderr, false)
            .unwrap_or_else(|error| panic!("{error}"));
    }

    let reference = Command::new(smlnj)
        .arg(fixture)
        .current_dir(directory)
        .output()
        .expect("run SML/NJ");
    if !valid
        || fixture.parent().and_then(Path::file_name) == Some(std::ffi::OsStr::new("lists"))
        || is_lexer_fixture(fixture)
    {
        assert_eq!(
            reference.status.success(),
            valid,
            "SML/NJ disagreed on {}: {}",
            fixture.display(),
            String::from_utf8_lossy(&reference.stdout)
        );
    }

    if is_lexer_fixture(fixture) && valid {
        let expected = fs::read(fixture.with_extension("tokens")).unwrap_or_else(|error| {
            panic!("read token snapshot for {}: {error}", fixture.display())
        });
        assert_eq!(
            nassau.stdout,
            expected,
            "tokens differ for {}",
            fixture.display()
        );
        return;
    }

    if valid {
        let executable = fixture.with_extension("");
        let nassau_output = Command::new(&executable)
            .current_dir(directory)
            .output()
            .expect("run Nassau output");
        let _ = fs::remove_file(&executable);
        check_program(fixture, &source, &nassau_output);
        if fixture.parent().and_then(Path::file_name) == Some(std::ffi::OsStr::new("lists")) {
            assert!(nassau_output.status.success(), "{}", fixture.display());
        } else {
            assert_eq!(
                nassau_output.stdout,
                smlnj_program_output(&reference.stdout, fixture),
                "stdout differs for {}",
                fixture.display()
            );
            assert_eq!(
                nassau_output.status.code(),
                reference.status.code(),
                "exit status differs for {}",
                fixture.display()
            );
        }
    }
}

fn main() {
    let arguments = Arguments::from_args();
    let explicit_smlnj = std::env::var_os("SMLNJ").is_some();
    let smlnj = std::env::var("SMLNJ").unwrap_or_else(|_| "smlnj".into());
    let smlnj_available = match Command::new(&smlnj).arg("-h").output() {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !explicit_smlnj => false,
        Err(error) => panic!("could not start SML/NJ ({smlnj}): {error}"),
    };
    let mlton_explicit = std::env::var_os("MLTON").is_some();
    let mlton = std::env::var("MLTON").unwrap_or_else(|_| "mlton".into());
    let mlton_available = match Command::new(&mlton).output() {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !mlton_explicit => false,
        Err(error) => panic!("could not start MLton ({mlton}): {error}"),
    };
    let fixtures_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let trials = fixtures()
        .into_iter()
        .map(|fixture| {
            let name = fixture
                .strip_prefix(&fixtures_root)
                .expect("fixture is under tests/fixtures")
                .display()
                .to_string();
            let smlnj = smlnj.clone();
            let mlton = mlton.clone();
            Trial::ignorable_test(name, move || {
                if !smlnj_available {
                    return Ok(Completion::ignored_with("SML/NJ is not installed"));
                }
                compare_fixture(&fixture, &smlnj);
                if mlton_available {
                    compare_mlton(&fixture, &mlton, expected_valid(&fixture));
                }
                Ok(Completion::Completed)
            })
        })
        .collect();
    libtest_mimic::run(&arguments, trials).exit();
}
