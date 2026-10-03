# Development

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

The REPL integration test validates inputs with Poly/ML and checks Nassau's
transcripts with FileCheck. Run it with:

```sh
cargo test --test repl
```

`(* ORACLE-VALUES *)` compares binding echoes with Poly/ML; `(* ORACLE-REPL *)`
compares the complete declaration transcript. Both ignore layout whitespace
outside strings. `CHECK-STDOUT` assertions in a REPL fixture check program
output in both Nassau and Poly/ML.

The tests use `poly` from `PATH` (override with `POLYML`) and skip if it is
not installed. SML/NJ and MLton are not required.

## Fixture expectations

Fixtures carry their expected output as FileCheck comments at the end of the
file (`CHECK-EXIT`, `CHECK-STDOUT`, `CHECK-STDERR` for compiled programs,
`CHECK-ERR` for fixtures under an `error/` directory, `CHECK-STDOUT` for the inferred types of `tests/fixtures/types` (every node's under `types/nodes`) and the core IR of `tests/fixtures/core`, and `CHECK-REPL` for `tests/repl`). LLVM's
`FileCheck` must be on `PATH` (or set `FILECHECK`). Regenerate them from
Poly/ML with:

```sh
tools/update_filecheck.py                     # every fixture
tools/update_filecheck.py 'tests/repl/*.sml'  # a glob or a single file
tools/update_filecheck.py --check             # fail if any block is stale
```

Valid type, parser, and core fixtures also compile and run as separate
`runtime/<fixture>` trials. Their `CHECK-RUN-EXIT`, `CHECK-RUN-STDOUT`, and
`CHECK-RUN-STDERR` blocks coexist with the original dump checks; the updater
generates program output and exit status from Poly/ML. Run these trials with
`cargo test --test fixtures runtime/`.

`(* RUNTIME-SKIP: reason *)` documents a fixture the backend cannot compile.
Its `CHECK-RUN-ERR` lines verify the current rejection. Both the harness and
updater fail if it begins compiling, requiring removal of the exclusion and
regeneration of runtime checks. The seven remaining exclusions are tracked in
`nassau-949.12`: infix datatype constructors, list append, word arithmetic,
first-class Basis functions, and `List` applications.

A `(* XFAIL: reason *)` line marks a REPL fixture whose output is known to
differ from the expected Nassau transcript; it fails the suite once it starts passing.

Compiled fixtures compare stdout and exit status against Poly/ML. The shared
[oracle driver](../tools/polyml_oracle.sml) suppresses binding echoes and separates
compiler warnings from program output. `(* POLYML-WARNING: text *)` checks an
oracle warning for a fixture Nassau rejects as an error, such as a redundant
match. `(* POLYML-SKIP: reason *)` excludes an oracle comparison for a documented
Poly/ML bug; Nassau's FileCheck checks still run.

`(* ORACLE-INT-PRECISION: 31 *)` marks a fixture that depends on Nassau's integer
width. The harness probes `Int.precision` and compares with Poly/ML only when it
matches. Nassau's FileCheck checks always run. The updater validates and retains
existing runtime checks when the oracle width differs.

The updater generates program stdout and exit checks from Poly/ML, and compiler
diagnostics and IR dumps from Nassau. Runtime diagnostic wording and REPL value
printing are Nassau-specific: the updater validates their existing FileCheck
checks and retains them. Edit these checks explicitly when changing the printer. REPL fixtures with an
uncaught exception declare `(* ORACLE-EXIT: 1 *)`.
