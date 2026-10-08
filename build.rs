use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=basis");
    let out = PathBuf::from(env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
    let sources = std::fs::read_to_string("basis/sources").expect("read basis source manifest");
    let mut basis = String::new();
    for source in sources.lines().filter(|line| !line.trim().is_empty()) {
        let path = PathBuf::from("basis").join(source);
        basis.push_str(
            &std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read basis source {}: {error}", path.display())),
        );
        basis.push('\n');
    }
    std::fs::write(out.join("basis.sml"), basis).expect("embed basis sources");
    if env::var_os("CARGO_FEATURE_NATIVE").is_none() {
        return;
    }
    println!("cargo:rerun-if-changed=runtime");
    println!("cargo:rerun-if-changed=src/value.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    let runtime_target = out.join("runtime-target");
    let target = env::var("TARGET").expect("cargo sets TARGET");
    let cargo = env::var_os("CARGO").expect("cargo sets CARGO");
    let result = Command::new(cargo)
        .args([
            "rustc",
            "--package",
            "nassau-runtime",
            "--lib",
            "--release",
            "--locked",
        ])
        .args(["--target", &target, "--target-dir"])
        .arg(&runtime_target)
        .args(["--", "--print=native-static-libs", "-D", "warnings"])
        .env("CARGO_PROFILE_RELEASE_PANIC", "abort")
        .output()
        .expect("build the native runtime archive");
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        result.status.success(),
        "native runtime build failed:\n{stderr}"
    );
    let libraries = stderr
        .lines()
        .find_map(|line| line.strip_prefix("note: native-static-libs: "))
        .expect("rustc reports native libraries for the runtime staticlib");
    println!("cargo:rustc-env=NASSAU_RUNTIME_NATIVE_LIBS={libraries}");
    std::fs::copy(
        runtime_target
            .join(target)
            .join("release/libnassau_runtime.a"),
        out.join("libnassau_runtime.a"),
    )
    .expect("embed the native runtime archive");
}
