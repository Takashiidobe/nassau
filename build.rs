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
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    let profile_dir = out
        .ancestors()
        .find(|path| path.file_name().is_some_and(|name| name == "build"))
        .and_then(std::path::Path::parent)
        .expect("OUT_DIR sits under the profile directory");
    let bootstrap_target = profile_dir.join("nassau-bootstrap");
    let target = env::var("TARGET").expect("cargo sets TARGET");
    let cargo = env::var_os("CARGO").expect("cargo sets CARGO");
    let mut command = Command::new(cargo);
    for (name, _) in env::vars_os() {
        if name.to_string_lossy().starts_with("CARGO_FEATURE_") {
            command.env_remove(name);
        }
    }
    let result = command
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .args([
            "rustc",
            "--package",
            "nassau",
            "--lib",
            "--crate-type",
            "staticlib",
            "--no-default-features",
            "--features",
            "runtime-archive",
            "--release",
            "--locked",
        ])
        .arg("--manifest-path")
        .arg(
            PathBuf::from(
                env::var_os("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"),
            )
            .join("Cargo.toml"),
        )
        .args(["--target", &target, "--target-dir"])
        .arg(&bootstrap_target)
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
        bootstrap_target.join(&target).join("release/libnassau.a"),
        out.join("libnassau_runtime.a"),
    )
    .expect("embed the native runtime archive");
    build_basis(&out, &target, &bootstrap_target);
}

fn build_basis(out: &std::path::Path, target: &str, bootstrap_target: &std::path::Path) {
    let architecture = target.split('-').next().expect("target architecture");
    let backend = match architecture {
        "x86_64" | "i686" | "i586" => "x86",
        "aarch64" | "aarch64_be" => "arm64",
        "s390x" => "s390x",
        "riscv64" | "riscv64gc" => "riscv64",
        _ => panic!("unsupported native Basis target: {target}"),
    };
    let features = format!("basis-bootstrap,cranelift-codegen/{backend}");
    let mut command = Command::new(env::var_os("CARGO").expect("cargo sets CARGO"));
    for (name, _) in env::vars_os() {
        if name.to_string_lossy().starts_with("CARGO_FEATURE_") {
            command.env_remove(name);
        }
    }
    let result = command
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .args([
            "run",
            "--bin",
            "nassau-basis",
            "--no-default-features",
            "--features",
        ])
        .arg(features)
        .args(["--profile", "bootstrap", "--locked", "--manifest-path"])
        .arg(PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml"))
        .arg("--target")
        .arg(env::var("HOST").expect("cargo sets HOST"))
        .arg("--target-dir")
        .arg(bootstrap_target)
        .arg("--")
        .arg(target)
        .arg(out)
        .output()
        .expect("build the compiled Basis");
    assert!(
        result.status.success(),
        "Basis build failed:\n{}",
        String::from_utf8_lossy(&result.stderr)
    );
    cc::Build::new()
        .object(out.join("basis.o"))
        .compile("nassau_basis");
}
