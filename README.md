# Nassau

A wip sml implementation in rust with a cranelift backend

To inspect Cranelift's compilation stages:

```sh
cargo run -- --debug-passes tests/fixtures/ret-42.sml
```

This prints the function IR before and after Cranelift optimization, followed by the lowered target instructions.

To emit the target instruction listing to `ret-42.S` instead of linking a binary:

```sh
cargo run -- --asm tests/fixtures/ret-42.sml
```

The `.S` file contains Cranelift's target-specific textual instruction listing, including its debug pseudo-instructions; it is intended for inspection rather than guaranteed to be directly re-assemblable.

Additional backend diagnostics are available through `clap`:

```sh
cargo run -- --dump-ir input.ml
cargo run -- --dump-optimized-ir --verify input.ml
cargo run -- --opt-level speed-and-size --stats --timings input.ml
cargo run -- --objdump input.ml
```

Earlier stages have their own dumps: `--dump-expr-types` prints the inferred type of every expression and pattern, and `--dump-core` prints the core IR a program lowers to before Cranelift (`src/core.rs`).

`--objdump` disassembles the actual Cranelift object bytes and requires `objdump` to be installed. `--debug-passes` remains as a shortcut for the main IR and instruction dumps.

The REPL integration test compares Nassau sessions with SML/NJ. Run it with:

```sh
cargo test --test repl
```

The test uses `smlnj` from `PATH` and skips if `smlnj` is not installed.

## Fixture expectations

Fixtures carry their expected output as FileCheck comments at the end of the
file (`CHECK-EXIT`, `CHECK-STDOUT`, `CHECK-STDERR` for compiled programs,
`CHECK-ERR` for fixtures under an `error/` directory, `CHECK-STDOUT` for the inferred types of `tests/fixtures/types` (every node's under `types/nodes`) and the core IR of `tests/fixtures/core`, and `CHECK-REPL` for `tests/repl`). LLVM's
`FileCheck` must be on `PATH` (or set `FILECHECK`). Regenerate them from
SML/NJ with:

```sh
tools/update_filecheck.py                     # every fixture
tools/update_filecheck.py 'tests/repl/*.sml'  # a glob or a single file
tools/update_filecheck.py --check             # fail if any block is stale
```

A `(* XFAIL: reason *)` line marks a REPL fixture whose output is known to
differ from SML/NJ; it fails the suite once it starts passing.

Compiled fixtures compare against SML/NJ. `(* SMLNJ-SKIP: reason *)` excludes
an oracle comparison for a documented SML/NJ bug. Nassau's FileCheck checks
still run. The updater validates and retains existing runtime expectations
for these fixtures, and regenerates Nassau diagnostics for error fixtures.
The exception-values and opaque functor-result fixtures carry this annotation
because SML/NJ 110.99.9 crashes on the former and incorrectly accepts the latter.

`(* SMLNJ-INT-PRECISION: 31 *)` marks a fixture that depends on Nassau's integer
width. The harness probes `Int.precision` and compares with SML/NJ only when it
matches. Nassau's FileCheck checks still run on every marked fixture. When the
oracle has a different width, the updater validates and retains the existing
runtime checks from the compatible oracle, and regenerates Nassau diagnostics
for error fixtures.

## Project documentation

[Language support and grammar](docs/grammar.md) describes what the parser,
type checker, native compiler, and REPL currently support, including the gaps.
[Architecture](docs/architecture.md) follows the compilation pipeline and
explains the core IR, generated helpers, support runtime, and REPL state.
[Value representation](docs/value-representation.md) specifies the tags and
heap layouts shared by generated code and the runtime.
