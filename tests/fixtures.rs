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

fn polyml_precision(polyml: &str) -> u32 {
    let mut child = Command::new(polyml)
        .arg("-q")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("probe Poly/ML integer precision");
    child.stdin.take().unwrap().write_all(
        b"val _ = print (\"NASSAU-INT-PRECISION: \" ^ (case Int.precision of SOME n => Int.toString n | NONE => \"0\") ^ \"\\n\");\n",
    ).expect("write Poly/ML precision probe");
    let output = child
        .wait_with_output()
        .expect("wait for Poly/ML precision");
    assert!(output.status.success(), "Poly/ML precision probe failed");
    String::from_utf8_lossy(&output.stdout)
        .split("NASSAU-INT-PRECISION: ")
        .nth(1)
        .and_then(|rest| rest.lines().next())
        .and_then(|number| number.trim().parse().ok())
        .expect("Poly/ML reports Int.precision")
}

fn matching_precision(source: &str, precision: u32) -> bool {
    directive(source, "ORACLE-INT-PRECISION").is_none_or(|required| {
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

/// Parser snapshots use `--dump-ast`; runtime trials compile the same source.
fn is_parser_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "parser")
}

/// Core snapshots use `--dump-core`; runtime trials compile the same source.
fn is_core_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "core")
}

/// Type snapshots use dumps; runtime trials compile the same source.
fn is_types_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "types")
}

fn is_lists_fixture(fixture: &Path) -> bool {
    in_directory(fixture, "lists")
}

fn polyml_program(polyml: &str, fixture: &Path) -> Output {
    Command::new(polyml)
        .args(["-q", "--script"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tools/polyml_oracle.sml"))
        .env("NASSAU_ORACLE_FILE", fixture)
        .current_dir(fixture.parent().expect("fixture has a parent directory"))
        .stdin(std::process::Stdio::null())
        .output()
        .expect("run Poly/ML fixture oracle")
}

fn check_program(fixture: &Path, source: &str, output: &Output, prefix: &str) {
    let code = output.status.code();
    if source.contains(&format!("(* {prefix}-EXIT")) {
        let code = code.map_or_else(|| "signal".to_owned(), |code| code.to_string());
        common::check_stream(
            fixture,
            source,
            &format!("{prefix}-EXIT"),
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
    common::check_stream(
        fixture,
        source,
        &format!("{prefix}-STDOUT"),
        &output.stdout,
        true,
    )
    .unwrap_or_else(|error| panic!("{error}"));
    common::check_stream(
        fixture,
        source,
        &format!("{prefix}-STDERR"),
        &output.stderr,
        true,
    )
    .unwrap_or_else(|error| panic!("{error}"));
}

fn compare_fixture(fixture: &Path, polyml: &str, precision: u32) {
    let valid = expected_valid(fixture);
    let source = fs::read_to_string(fixture).expect("read fixture");
    let skip_reason = directive(&source, "POLYML-SKIP");
    let compare_oracle = skip_reason.is_none() && matching_precision(&source, precision);
    if !compare_oracle {
        let reason = skip_reason
            .map(str::to_owned)
            .unwrap_or_else(|| format!("Int.precision is {precision}"));
        eprintln!(
            "{}: skipping Poly/ML ({reason}); running Nassau FileCheck checks only",
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
        let reference = polyml_program(polyml, fixture);
        let warning = directive(&source, "POLYML-WARNING");
        let accepted = reference.status.success()
            && !warning.is_some_and(|message| {
                String::from_utf8_lossy(&reference.stderr).contains(message)
            });
        assert_eq!(
            accepted,
            valid,
            "Poly/ML disagreed on {}: {}{}",
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
        let mut command = Command::new(&executable);
        common::configure_gc(&mut command, &source);
        let nassau_output = command
            .current_dir(directory)
            .output()
            .expect("run Nassau output");
        let _ = fs::remove_file(&executable);
        check_program(fixture, &source, &nassau_output, "CHECK");
        if is_lists_fixture(fixture) {
            assert!(nassau_output.status.success(), "{}", fixture.display());
        } else if compare_oracle {
            let reference = polyml_program(polyml, fixture);
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

fn runtime_fixture(fixture: &Path, polyml: &str, precision: u32) {
    let source = fs::read_to_string(fixture).expect("read fixture");
    let directory = fixture.parent().expect("fixture parent");
    let compiled = Command::new(env!("CARGO_BIN_EXE_nassau"))
        .arg("--verify")
        .arg(fixture)
        .current_dir(directory)
        .output()
        .expect("compile runtime fixture");
    if let Some(reason) = directive(&source, "RUNTIME-SKIP") {
        if compiled.status.success() {
            let _ = fs::remove_file(fixture.with_extension(""));
            panic!(
                "{} now compiles; remove RUNTIME-SKIP ({reason}) and generate runtime checks",
                fixture.display()
            );
        }
        common::check_stream(fixture, &source, "CHECK-RUN-ERR", &compiled.stderr, false)
            .unwrap_or_else(|error| panic!("{error}"));
        return;
    }
    assert!(
        compiled.status.success(),
        "{}: runtime compilation failed: {}",
        fixture.display(),
        String::from_utf8_lossy(&compiled.stderr)
    );
    let executable = fixture.with_extension("");
    let mut command = Command::new(&executable);
    common::configure_gc(&mut command, &source);
    let output = command.current_dir(directory).output();
    let _ = fs::remove_file(&executable);
    let output = output.expect("run runtime fixture");
    assert!(
        source.contains("(* CHECK-RUN-EXIT:"),
        "{}: missing runtime expectations; run tools/update_filecheck.py",
        fixture.display()
    );
    check_program(fixture, &source, &output, "CHECK-RUN");
    if directive(&source, "POLYML-SKIP").is_none() && matching_precision(&source, precision) {
        let reference = polyml_program(polyml, fixture);
        assert_eq!(
            output.stdout,
            reference.stdout,
            "runtime stdout differs for {}",
            fixture.display()
        );
        assert_eq!(
            output.status.code(),
            reference.status.code(),
            "runtime exit differs for {}",
            fixture.display()
        );
    }
}

fn main() {
    let arguments = Arguments::from_args();
    let explicit_polyml = std::env::var_os("POLYML").is_some();
    let polyml = std::env::var("POLYML").unwrap_or_else(|_| "poly".into());
    let polyml_available = match Command::new(&polyml).arg("--help").output() {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !explicit_polyml => false,
        Err(error) => panic!("could not start Poly/ML ({polyml}): {error}"),
    };
    let precision = if polyml_available {
        polyml_precision(&polyml)
    } else {
        0
    };
    let fixtures_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut trials = Vec::new();
    for fixture in fixtures() {
        let name = fixture
            .strip_prefix(&fixtures_root)
            .expect("fixture root")
            .display()
            .to_string();
        if expected_valid(&fixture)
            && (is_types_fixture(&fixture)
                || is_parser_fixture(&fixture)
                || is_core_fixture(&fixture))
        {
            let runtime_path = fixture.clone();
            let runtime_polyml = polyml.clone();
            trials.push(Trial::ignorable_test(
                format!("runtime/{name}"),
                move || {
                    if !polyml_available {
                        return Ok(Completion::ignored_with("Poly/ML is not installed"));
                    }
                    runtime_fixture(&runtime_path, &runtime_polyml, precision);
                    Ok(Completion::Completed)
                },
            ));
        }
        let polyml = polyml.clone();
        trials.push(Trial::ignorable_test(name, move || {
            if !polyml_available {
                return Ok(Completion::ignored_with("Poly/ML is not installed"));
            }
            compare_fixture(&fixture, &polyml, precision);
            Ok(Completion::Completed)
        }));
    }

    libtest_mimic::run(&arguments, trials).exit();
}
