# Basis libraries

`sources` lists the SML files in dependency order. Add each new library there
after the libraries it uses. `build.rs` concatenates and embeds these sources
for both native and web builds, so adding a library needs no Rust changes.

Native builds precompile the Basis for the target platform and link it into
Nassau. Changes to the Basis rebuild this code automatically. REPL startup and
reset initialize the bundled code without running Cranelift over the Basis.

Run `cargo package --allow-dirty` to verify that the published sources can build
the bundled Basis without files from your checkout.
