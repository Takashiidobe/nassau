//! Shared FileCheck support for the fixture harnesses.
//!
//! Expectations live in the fixture as SML comment lines such as
//! `(* CHECK-STDOUT: hello *)`; `tools/update_filecheck.py` generates them.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_FILE: AtomicUsize = AtomicUsize::new(0);

/// The FileCheck directives for `prefix` (e.g. `CHECK-STDOUT`), unwrapped from
/// their comment, or none if the fixture makes no assertion about that stream.
fn directives(source: &str, prefix: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("(* ")?
                .strip_suffix(" *)")
                .map(str::to_owned)
        })
        .filter(|text| {
            text.strip_prefix(prefix)
                .is_some_and(|rest| rest.starts_with(':') || rest.starts_with('-'))
        })
        .collect()
}

/// Checks `stream` against the fixture's `prefix` directives with FileCheck.
/// A stream the fixture says nothing about must be empty.
pub fn check_stream(
    fixture: &Path,
    source: &str,
    prefix: &str,
    stream: &[u8],
    full_lines: bool,
) -> Result<(), String> {
    let lines = directives(source, prefix);
    if lines.is_empty() {
        return if stream.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{}: unexpected output with no {prefix} lines (run tools/update_filecheck.py):\n{}",
                fixture.display(),
                String::from_utf8_lossy(stream)
            ))
        };
    }

    let check_file = std::env::temp_dir().join(format!(
        "nassau-filecheck-{}-{}.txt",
        std::process::id(),
        NEXT_FILE.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&check_file, lines.join("\n") + "\n").expect("write FileCheck check file");

    let filecheck = std::env::var("FILECHECK").unwrap_or_else(|_| "FileCheck".into());
    let mut command = Command::new(&filecheck);
    command
        .arg(&check_file)
        .arg(format!("--check-prefix={prefix}"))
        .arg("--allow-empty");
    if full_lines {
        command.arg("--match-full-lines");
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| {
            panic!("could not start FileCheck ({filecheck}); install LLVM's FileCheck or set FILECHECK: {error}")
        });
    // FileCheck may exit before reading all of stdin; a broken pipe is fine.
    let _ = child.stdin.take().expect("piped stdin").write_all(stream);
    let output = child.wait_with_output().expect("wait for FileCheck");
    let _ = std::fs::remove_file(&check_file);

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{}: {prefix} does not match:\n{}",
            fixture.display(),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
