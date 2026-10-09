# Development

## Inspect compilation

```sh
cargo run -- --debug-passes tests/fixtures/ret-42.sml
```

This prints the Cranelift IR before and after optimization, then the target
instructions. Other dumps:

```sh
cargo run -- --asm input.sml                  # write the listing to input.S
cargo run -- --dump-ir input.sml
cargo run -- --dump-optimized-ir --verify input.sml
cargo run -- --opt-level speed-and-size --stats --timings input.sml
cargo run -- --objdump input.sml              # needs objdump installed
cargo run -- --dump-expr-types input.sml      # type of every expression and pattern
cargo run -- --dump-core input.sml            # core IR; skips the basis
```

The `.S` file is for reading; it may not reassemble. `--dump-core` can't show a
program that uses a basis structure such as `List`.

## Run the tests

Tests are SML fixtures, not unit tests:

- `tests/fixtures`: compiled programs, parser, type, and core-IR checks.
- `tests/repl`: REPL transcripts.

```sh
cargo nextest r --release
cargo test --test fixtures runtime/   # run valid fixtures as native programs
cargo test --test repl                # REPL transcripts
```

Requirements:

- `poly` on `PATH` (or `POLYML`) is the oracle. Oracle comparisons are skipped
  if it's missing.
- LLVM's `FileCheck` on `PATH` (or `FILECHECK`).

The browser session fixture runs the generated Wasm in Node and compares its
REPL transcripts with the native JIT:

```sh
cargo build --release
./www/build.sh
node tests/fixtures/runtime/browser-session.mjs
```

## Fixture expectations

Expected output is FileCheck comments at the end of the fixture:

| Directive                                                | Applies to                                          |
| -------------------------------------------------------- | --------------------------------------------------- |
| `CHECK-EXIT`, `CHECK-STDOUT`, `CHECK-STDERR`             | Compiled programs                                   |
| `CHECK-ERR`                                              | Fixtures under an `error/` directory                |
| `CHECK-STDOUT`                                           | `tests/fixtures/types`, `types/nodes`, and `core`   |
| `CHECK-REPL`                                             | `tests/repl`                                        |
| `CHECK-RUN-EXIT`, `CHECK-RUN-STDOUT`, `CHECK-RUN-STDERR` | Native run of a valid type, parser, or core fixture |

Regenerate expectations from Poly/ML:

```sh
tools/update_filecheck.py                     # every fixture
tools/update_filecheck.py 'tests/repl/*.sml'  # a glob or a single file
tools/update_filecheck.py --check             # fail if any block is stale
```

The updater takes program output and exit status from Poly/ML, and compiler
diagnostics and IR dumps from Nassau. Runtime diagnostic wording and REPL value
printing are Nassau-specific, so the updater only validates their existing
checks. Edit those by hand when you change the printer.

### Fixture comments

| Comment                                 | Effect                                                      |
| --------------------------------------- | ----------------------------------------------------------- |
| `(* ORACLE-VALUES *)`                   | REPL: compare binding echoes with Poly/ML                   |
| `(* ORACLE-REPL *)`                     | REPL: compare the whole declaration transcript with Poly/ML |
| `(* ORACLE-EXIT: 1 *)`                  | REPL: expect exit status 1, for an uncaught exception       |
| `(* ORACLE-INT-PRECISION: 31 *)`        | Compare with Poly/ML only if its `Int.precision` matches    |
| `(* POLYML-SKIP: reason *)`             | Skip the oracle comparison for a known Poly/ML bug          |
| `(* POLYML-WARNING: text *)`            | Expect an oracle warning where Nassau gives an error        |
| `(* RUNTIME-SKIP: reason *)`            | Skip native compile; `CHECK-RUN-ERR` checks the rejection   |
| `(* XFAIL: reason *)`                   | REPL: output is known to differ; fails once it passes       |
| `(* REPL-COMMANDS *)`                   | REPL: test host commands without Poly/ML                    |
| `(* GC-PLAN *)`, `GC-HEAP`, `GC-STRESS` | Set the runtime GC options for Nassau only                  |

Comparisons ignore layout whitespace outside strings. Nassau's FileCheck checks
always run, even when an oracle comparison is skipped. A fixture with
`RUNTIME-SKIP` or `XFAIL` fails once it starts passing, so remove the marker and
regenerate.

### Unsupported grammar

`tests/fixtures/grammar` has one fixture per SML'97 production. Productions
Nassau doesn't handle yet live in `tests/fixtures/grammar.unsupported`, with the
tracking bead named in the header. Poly/ML must accept them and Nassau must
fail. When one starts passing, its trial fails and prints the `git mv` that
moves it into `tests/fixtures/grammar`. Any `*.unsupported/` directory works
this way.

### External suites

```sh
tools/import_suite.py <suite> --source <checkout>
```

Import at a pinned upstream commit, with Poly/ML as the only oracle. Each suite
keeps its licence and `PROVENANCE` in `tests/fixtures/<suite>/`, and the first
line of every file names its upstream path and commit. Programs are sorted by
how Poly/ML and Nassau compare:

| Directory                    | Poly/ML                  | Nassau      |
| ---------------------------- | ------------------------ | ----------- |
| `<suite>/`                   | accepts                  | same output |
| `<suite>/error/`             | rejects                  | rejects     |
| `<suite>.unsupported/`       | accepts                  | differs     |
| `<suite>.unsupported/error/` | rejects                  | accepts     |
| `<suite>.ignored/`           | disagrees with the suite | never run   |

`<suite>/EXCLUDED.tsv` lists programs skipped for MLton primitives, FFI imports,
or `use`.

## Garbage collector

The `runtime` crate wraps MMTk. The compiler and JIT link it as an rlib;
generated programs link it as a staticlib embedded in the compiler. It requires
`std`. Only x86_64 Linux is tested.

The default is non-moving MarkSweep with a 32 MiB heap. Set these environment
variables to change it:

| Variable           | Effect                                             | Default   |
| ------------------ | -------------------------------------------------- | --------- |
| `NASSAU_GC_PLAN`   | `NoGC` selects allocation-only, with no collection | MarkSweep |
| `NASSAU_GC_HEAP`   | Heap size, such as `8m`                            | `32m`     |
| `NASSAU_GC_STRESS` | Collect before every Nth allocation; 0 disables    | `0`       |
| `NASSAU_CC`        | C compiler used to link native programs            | `cc`      |

Running out of memory prints `nassau: out of memory` and exits with status 1.
SML runtime globals assume a single SML thread.

The collector finds roots through shadow-stack frames that generated code
publishes, runtime `with_roots` scopes, visible REPL globals and structure
exports, and the globals that live closures and functors depend on. Code
generated for the JIT is kept for the process lifetime.

Run the allocation and object-model fixtures:

```sh
cargo test --test fixtures mmtk-nogc
cargo test --test repl
cargo test -p nassau-runtime --test object-model
```

The GC-specific fixtures:

- `mmtk-nogc.sml`: the NoGC plan.
- `mmtk-collecting.sml`, `mmtk-history.sml`, `mmtk-exhaustion.sml`: production
  collection and exhaustion, each under its own small heap.
- `mmtk-safepoints.sml`: forces a collection at every allocation.

Run the full suite with periodic forced collection:

```sh
NASSAU_GC_STRESS=1000 NASSAU_GC_HEAP=32m cargo test --workspace
```

Don't use a stress interval of 1 on fixtures that build long lists. GC work
becomes quadratic.
