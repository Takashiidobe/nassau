//! Builds the runtime library (runtime/nassau_runtime.c) with the system C
//! compiler. The compiler links it in for the REPL's JIT and embeds the
//! archive to link into compiled programs.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn run(command: &mut Command) {
    let status = command
        .status()
        .unwrap_or_else(|error| panic!("failed to run {command:?}: {error}"));
    assert!(status.success(), "{command:?} failed with {status}");
}

fn main() {
    let source = "runtime/nassau_runtime.c";
    println!("cargo:rerun-if-changed={source}");
    println!("cargo:rerun-if-env-changed=NASSAU_CC");
    let out = PathBuf::from(env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
    let object = out.join("nassau_runtime.o");
    let archive = out.join("libnassau_runtime.a");
    let cc = env::var("NASSAU_CC").unwrap_or_else(|_| "cc".to_string());
    run(Command::new(&cc)
        .args(["-c", "-O2", "-fPIC", "-std=c99", "-Wall", "-Werror", "-o"])
        .arg(&object)
        .arg(source));
    let _ = std::fs::remove_file(&archive);
    run(Command::new("ar").arg("crs").arg(&archive).arg(&object));
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=nassau_runtime");
}
