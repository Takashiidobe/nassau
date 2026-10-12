# Basis libraries

`sources` lists the SML files in dependency order. Add each new library there
after the libraries it uses. `build.rs` concatenates and embeds these sources
for both native and web builds, so adding a library needs no Rust changes.

Native builds precompile the Basis for the target platform and link it into
Nassau. Changes to the Basis rebuild this code automatically. REPL startup and
reset initialize the bundled code without running Cranelift over the Basis.

`build.rs` builds the runtime archive and the Basis in one shared nested target,
`target/<profile>/nassau-bootstrap`, so dependencies compile once and compiler
edits only rebuild the Nassau crate. The bootstrap compiler uses the
`bootstrap` profile in `Cargo.toml`.

Run `cargo package --allow-dirty` to verify that the published sources can build
the bundled Basis without files from your checkout.

The compiled Basis is cached under `target/<profile>/nassau-bootstrap/basis-<target>/`,
keyed by a hash of `basis/`, `src/`, `runtime/`, `build.rs`, `Cargo.toml` and
`Cargo.lock`. Edits to `src/repl.rs`, `main.rs`, `web.rs` and `ast_dump.rs` do
not change the key, so they skip the Basis bootstrap. New files under `src/`
count towards the key unless added to `NOT_IN_BASIS_KEY` in `build.rs`.
