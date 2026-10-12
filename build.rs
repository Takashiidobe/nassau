use std::env;
use std::path::{Path, PathBuf};
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
        .env("CARGO_TERM_COLOR", "never")
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

const BASIS_OUTPUTS: [&str; 2] = ["basis.o", "basis_symbols.rs"];
const NOT_IN_BASIS_KEY: [&str; 4] = ["repl.rs", "main.rs", "web.rs", "ast_dump.rs"];

fn hash_file(hash: &mut u64, path: &Path) {
    let name = path.to_string_lossy();
    let bytes = std::fs::read(path).unwrap_or_else(|error| panic!("read {name}: {error}"));
    for byte in name.bytes().chain([0]).chain(bytes) {
        *hash = (*hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
    }
}

fn hash_tree(hash: &mut u64, directory: &Path) {
    let mut entries: Vec<_> = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
        .map(|entry| entry.expect("read directory entry").path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            hash_tree(hash, &path);
        } else if !NOT_IN_BASIS_KEY.contains(&path.file_name().unwrap().to_str().unwrap()) {
            hash_file(hash, &path);
        }
    }
}

// everything the compiled Basis can depend on: new files under src/ count by default
fn basis_key() -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    for directory in ["basis", "src", "runtime"] {
        hash_tree(&mut hash, Path::new(directory));
    }
    for file in ["build.rs", "Cargo.toml", "Cargo.lock"] {
        hash_file(&mut hash, Path::new(file));
    }
    format!("{hash:016x}")
}

fn build_basis(out: &Path, target: &str, bootstrap_target: &Path) {
    let key = basis_key();
    let caches = bootstrap_target.join(format!("basis-{target}"));
    let cache = caches.join(&key);
    if cache.join("complete").exists() {
        for name in BASIS_OUTPUTS {
            std::fs::copy(cache.join(name), out.join(name)).expect("reuse the compiled Basis");
        }
        link_basis(out);
        return;
    }
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
        .env("CARGO_TERM_COLOR", "never")
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
    std::fs::create_dir_all(&cache).expect("create the Basis cache");
    for name in BASIS_OUTPUTS {
        std::fs::copy(out.join(name), cache.join(name)).expect("cache the compiled Basis");
    }
    std::fs::write(cache.join("complete"), "").expect("mark the Basis cache complete");
    prune_basis_caches(&caches);
    link_basis(out);
}

fn prune_basis_caches(caches: &Path) {
    let mut entries: Vec<_> = std::fs::read_dir(caches)
        .expect("read the Basis caches")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            Some((entry.metadata().ok()?.modified().ok()?, entry.path()))
        })
        .collect();
    entries.sort();
    let stale = entries.len().saturating_sub(8);
    for (_, path) in entries.into_iter().take(stale) {
        let _ = std::fs::remove_dir_all(path);
    }
}

fn link_basis(out: &Path) {
    cc::Build::new()
        .object(out.join("basis.o"))
        .compile("nassau_basis");
}
