# Nassau

A wip sml implementation in rust with a cranelift backend

To inspect Cranelift's compilation stages:

```sh
cargo run -- --debug-passes tests/ret-42.ml
```

This prints the function IR before and after Cranelift optimization, followed by the lowered target instructions.

To emit the target instruction listing to `ret-42.S` instead of linking a binary:

```sh
cargo run -- --asm tests/ret-42.ml
```

The `.S` file contains Cranelift's target-specific textual instruction listing, including its debug pseudo-instructions; it is intended for inspection rather than guaranteed to be directly re-assemblable.

Additional backend diagnostics are available through `clap`:

```sh
cargo run -- --dump-ir input.ml
cargo run -- --dump-optimized-ir --verify input.ml
cargo run -- --opt-level speed-and-size --stats --timings input.ml
cargo run -- --objdump input.ml
```

`--objdump` disassembles the actual Cranelift object bytes and requires `objdump` to be installed. `--debug-passes` remains as a shortcut for the main IR and instruction dumps.

The REPL integration test compares Nassau sessions with SML/NJ. Run it with:

```sh
cargo test --test repl
```

The test uses `smlnj` from `PATH` and skips if `smlnj` is not installed.

## Features

- Integer arithmetic supports `+`, `-`, `*`, and `div`.
- Real arithmetic supports `+`, `-`, `*`, and `/`.
- Conditionals like `If then else`
- Relops, CmpOps like `<=, >=, <, >, =, <>`
- Basic list construction
- `true` and `false`
