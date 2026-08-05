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
    let source = "runtime/nassau_runtime.rs";
    println!("cargo:rerun-if-changed={source}");
    println!("cargo:rerun-if-changed=src/value.rs");
    let out = PathBuf::from(env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
    let object = out.join("nassau_runtime.o");
    let archive = out.join("libnassau_runtime.a");
    let rustc = env::var_os("RUSTC").expect("cargo sets RUSTC");
    let target = env::var("TARGET").expect("cargo sets TARGET");
    run(Command::new(rustc)
        .args([
            "--edition=2024",
            "--crate-type=lib",
            "--emit=obj",
            "-O",
            "-D",
            "warnings",
        ])
        .args(["-C", "panic=abort", "-C", "relocation-model=pic"])
        .args(["-A", "dead_code", "--target", &target, "-o"])
        .arg(&object)
        .arg(source));
    let _ = std::fs::remove_file(&archive);
    run(Command::new("ar").arg("crs").arg(&archive).arg(&object));
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=nassau_runtime");
}
