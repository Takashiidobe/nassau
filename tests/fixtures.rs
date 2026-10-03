mod common;

use libtest_mimic::{Arguments, Completion, Trial};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn directive<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    source.lines().find_map(|line| {
        line.trim()
            .strip_prefix(&format!("(* {name}: "))?
            .strip_suffix(" *)")
    })
}

fn smlnj_precision(smlnj: &str) -> u32 {
    let mut child = Command::new(smlnj)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("probe SML/NJ integer precision");
    child.stdin.take().unwrap().write_all(
        b"val _ = print (\"NASSAU-INT-PRECISION: \" ^ (case Int.precision of SOME n => Int.toString n | NONE => \"0\") ^ \"\\n\");\n",
    ).expect("write SML/NJ precision probe");
    let output = child.wait_with_output().expect("wait for SML/NJ precision");
    assert!(output.status.success(), "SML/NJ precision probe failed");
    String::from_utf8_lossy(&output.stdout)
        .split("NASSAU-INT-PRECISION: ")
        .nth(1)
        .and_then(|rest| rest.lines().next())
        .and_then(|number| number.trim().parse().ok())
        .expect("SML/NJ reports Int.precision")
}

fn matching_precision(source: &str, precision: u32) -> bool {
    directive(source, "SMLNJ-INT-PRECISION").is_none_or(|required| {
        required.parse::<u32>().expect("fixture integer precision") == precision
    })
}

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

/// Whether a directory named `name` appears anywhere below `tests/fixtures`.
fn in_directory(fixture: &Path, name: &str) -> bool {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    fixture
        .strip_prefix(&root)
        .unwrap_or(fixture)
        .parent()
        .is_some_and(|directory| directory.iter().any(|part| part == name))
}

/// Fixtures under an `error/` directory must be rejected; all others accepted.
fn expected_valid(fixture: &Path) -> bool {
    !in_directory(fixture, "error")
}

fn is_lexer_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "lexer")
}

/// Parser fixtures are checked through `--dump-ast`, not compiled and run.
fn is_parser_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "parser")
}

/// Core fixtures are checked through `--dump-core`, not compiled and run.
fn is_core_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "core")
}

/// Type fixtures are checked through `--dump-types`, not compiled and run.
fn is_types_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "types")
}

fn is_lists_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "lists")
}

/// Runs a fixture under SML/NJ as tools/update_filecheck.py does: loaded
/// by base name with the echo of bindings silenced, so stdout is the
/// program's own output.
fn smlnj_program(smlnj: &str, fixture: &Path) -> Output {
    let name = fixture.file_name().expect("fixture has a file name");
    let wrapper = std::env::temp_dir().join(format!(
        "nassau-smlnj-{}-{}.sml",
        std::process::id(),
        name.to_string_lossy()
    ));
    fs::write(
        &wrapper,
        format!(
            "val _ = Control.Print.out := {{say = fn _ => (), flush = fn () => ()}};\nuse \"{}\";\n",
            name.to_string_lossy()
        ),
    )
    .expect("write SML/NJ wrapper");
    let mut output = Command::new(smlnj)
        .arg(&wrapper)
        .current_dir(fixture.parent().expect("fixture has a parent directory"))
        .stdin(std::process::Stdio::null())
        .output()
        .expect("run SML/NJ");
    let _ = fs::remove_file(&wrapper);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let noise = [
        "Standard ML of New Jersey",
        "[opening ",
        "[autoloading",
        "[library ",
        "[scanning ",
        "[parsing ",
    ];
    let mut lines: Vec<&str> = stdout.split('\n').collect();
    while lines
        .first()
        .is_some_and(|line| noise.iter().any(|prefix| line.starts_with(prefix)))
    {
        lines.remove(0);
    }
    output.stdout = lines.join("\n").into_bytes();
    output
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

fn compare_fixture(fixture: &Path, smlnj: &str, precision: u32) {
    let valid = expected_valid(fixture);
    let source = fs::read_to_string(fixture).expect("read fixture");
    let skip_reason = directive(&source, "SMLNJ-SKIP");
    let compare_oracle = skip_reason.is_none() && matching_precision(&source, precision);
    if !compare_oracle {
        let reason = skip_reason
            .map(str::to_owned)
            .unwrap_or_else(|| format!("Int.precision is {precision}"));
        eprintln!(
            "{}: skipping SML/NJ ({reason}); running Nassau FileCheck checks only",
            fixture.display()
        );
    }
    let directory = fixture.parent().expect("fixture has a parent directory");
    let mut nassau_command = Command::new(env!("CARGO_BIN_EXE_nassau"));
    if is_lexer_fixture(fixture) {
        nassau_command.arg("--dump-tokens");
    }
    if is_parser_fixture(fixture) {
        nassau_command.arg("--dump-ast");
    }
    if is_core_fixture(fixture) {
        nassau_command.arg("--dump-core");
    }
    if is_types_fixture(fixture) {
        // Fixtures under types/nodes list the type of every expression.
        if in_directory(fixture, "nodes") {
            nassau_command.arg("--dump-expr-types");
        } else {
            nassau_command.arg("--dump-types");
        }
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
    if !valid {
        common::check_stream(fixture, &source, "CHECK-ERR", &nassau.stderr, false)
            .unwrap_or_else(|error| panic!("{error}"));
    }

    if compare_oracle
        && (!valid
            || is_lists_fixture(fixture)
            || is_lexer_fixture(fixture)
            || is_parser_fixture(fixture)
            || is_core_fixture(fixture)
            || is_types_fixture(fixture))
    {
        let reference = Command::new(smlnj)
            .arg(fixture)
            .current_dir(directory)
            .stdin(std::process::Stdio::null())
            .output()
            .expect("run SML/NJ fixture oracle");
        assert_eq!(
            reference.status.success(),
            valid,
            "SML/NJ disagreed on {}: {}{}",
            fixture.display(),
            String::from_utf8_lossy(&reference.stdout),
            String::from_utf8_lossy(&reference.stderr)
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

    if (is_parser_fixture(fixture) || is_core_fixture(fixture) || is_types_fixture(fixture))
        && valid
    {
        common::check_stream(fixture, &source, "CHECK-STDOUT", &nassau.stdout, true)
            .unwrap_or_else(|error| panic!("{error}"));
        common::check_stream(fixture, &source, "CHECK-STDERR", &nassau.stderr, true)
            .unwrap_or_else(|error| panic!("{error}"));
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
        if is_lists_fixture(fixture) {
            assert!(nassau_output.status.success(), "{}", fixture.display());
        } else if compare_oracle {
            let reference = smlnj_program(smlnj, fixture);
            assert_eq!(
                nassau_output.stdout,
                reference.stdout,
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
    let precision = if smlnj_available {
        smlnj_precision(&smlnj)
    } else {
        0
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
            Trial::ignorable_test(name, move || {
                if !smlnj_available {
                    return Ok(Completion::ignored_with("SML/NJ is not installed"));
                }
                compare_fixture(&fixture, &smlnj, precision);
                Ok(Completion::Completed)
            })
        })
        .collect();
    libtest_mimic::run(&arguments, trials).exit();
}
