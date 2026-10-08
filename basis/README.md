# Basis libraries

`sources` lists the SML files in dependency order. Add each new library there
after the libraries it uses. `build.rs` concatenates and embeds these sources
for both native and web builds, so adding a library needs no Rust changes.
