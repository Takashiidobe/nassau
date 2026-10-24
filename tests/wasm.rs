use libtest_mimic::{Arguments, Failed, Trial};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXCLUDED: [&str; 7] = [
    "error",
    "lexer",
    "parser",
    "core",
    "types",
    "conformance.unsupported",
    "grammar.unsupported",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn bundle() -> PathBuf {
    std::env::var_os("NASSAU_WASM_BUNDLE")
        .map_or_else(|| root().join("www/dist/pkg"), PathBuf::from)
}

fn collect(directory: &Path, paths: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("read fixtures directory") {
        let path = entry.expect("read fixture entry").path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if path.is_dir() {
            if !EXCLUDED.contains(&name) && !name.ends_with(".ignored") {
                collect(&path, paths);
            }
        } else if path.extension().is_some_and(|extension| extension == "sml") {
            paths.push(path);
        }
    }
}

fn directives(source: &str, prefix: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| {
            let text = line.trim().strip_prefix("(* ")?.strip_suffix(" *)")?;
            let rest = text.strip_prefix(prefix)?;
            let rest = rest.strip_prefix("-NEXT").unwrap_or(rest);
            rest.strip_prefix(": ").map(str::to_owned)
        })
        .collect()
}

fn gc_environment(source: &str) -> String {
    let settings: Vec<String> = ["PLAN", "HEAP", "STRESS"]
        .iter()
        .filter_map(|option| {
            let prefix = format!("(* GC-{option}: ");
            let value = source
                .lines()
                .find_map(|line| line.trim().strip_prefix(&prefix)?.strip_suffix(" *)"))?;
            Some(format!("\"NASSAU_GC_{option}\":\"{value}\""))
        })
        .collect();
    format!("{{{}}}", settings.join(","))
}

fn run(fixture: &Path, prefix: &str) -> Result<(), Failed> {
    let source = fs::read_to_string(fixture).expect("read fixture");
    let output = Command::new("node")
        .arg(root().join("tests/wasm/run-fixture.mjs"))
        .arg(bundle())
        .arg(fixture)
        .arg(gc_environment(&source))
        .current_dir(fixture.parent().expect("fixture has a parent directory"))
        .output()
        .map_err(|error| format!("could not start node: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() && directives(&source, &format!("{prefix}-STDERR")).is_empty() {
        return Err(format!("unexpected diagnostics:\n{stderr}").into());
    }
    let expected_exit = directives(&source, &format!("{prefix}-EXIT"))
        .first()
        .and_then(|code| code.trim().parse().ok())
        .unwrap_or(0);
    if output.status.code() != Some(expected_exit) {
        return Err(format!("exit {:?}, expected {expected_exit}", output.status.code()).into());
    }
    let lines: Vec<&str> = stdout.lines().collect();
    let mut cursor = 0;
    for expected in directives(&source, &format!("{prefix}-STDOUT")) {
        let Some(found) = lines[cursor..]
            .iter()
            .position(|line| line.trim_end() == expected.trim_end())
        else {
            return Err(format!("missing \"{expected}\" in:\n{stdout}").into());
        };
        cursor += found + 1;
    }
    Ok(())
}

fn main() {
    let arguments = Arguments::from_args();
    let fixtures_root = root().join("tests/fixtures");
    let mut paths = Vec::new();
    collect(&fixtures_root, &mut paths);
    paths.sort();
    let bundle_ready = bundle().join("nassau.js").exists();
    let trials = paths
        .into_iter()
        .filter_map(|fixture| {
            let source = fs::read_to_string(&fixture).expect("read fixture");
            let prefix = if source.contains("(* CHECK-RUN-EXIT:") {
                "CHECK-RUN"
            } else {
                "CHECK"
            };
            let checked = !directives(&source, &format!("{prefix}-STDOUT")).is_empty();
            if !checked || source.contains("(* WASM-SKIP: ") {
                return None;
            }
            let name = fixture
                .strip_prefix(&fixtures_root)
                .expect("fixture root")
                .display()
                .to_string();
            Some(Trial::test(name, move || {
                if !bundle_ready {
                    return Err("no wasm bundle; run ./www/build.sh".into());
                }
                run(&fixture, prefix)
            }))
        })
        .collect();
    libtest_mimic::run(&arguments, trials).exit();
}
