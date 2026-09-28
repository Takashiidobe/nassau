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

## Fixture expectations

Fixtures carry their expected output as FileCheck comments at the end of the
file (`CHECK-EXIT`, `CHECK-STDOUT`, `CHECK-STDERR` for compiled programs,
`CHECK-ERR` for fixtures under an `error/` directory, and `CHECK-STDOUT` for the inferred types of `tests/fixtures/types`, `CHECK-REPL` for `tests/repl`). LLVM's
`FileCheck` must be on `PATH` (or set `FILECHECK`). Regenerate them from
SML/NJ with:

```sh
tools/update_filecheck.py                     # every fixture
tools/update_filecheck.py 'tests/repl/*.sml'  # a glob or a single file
tools/update_filecheck.py --check             # fail if any block is stale
```

A `(* XFAIL: reason *)` line marks a REPL fixture whose output is known to
differ from SML/NJ; it fails the suite once it starts passing.

## Features

- Integer arithmetic supports `+`, `-`, `*`, and `div`.
- Real arithmetic supports `+`, `-`, `*`, and `/`.
- Conditionals like `If then else`
- Relops, CmpOps like `<=, >=, <, >, =, <>`
- Basic list construction
- The parser accepts the full SML expression syntax (application, default infix fixities, tuples, records, selectors, sequences, `let`, `case`, `fn`, `while`, `raise`/`handle`, type annotations); `--dump-ast` prints the tree. Semantic analysis still rejects the forms it cannot check or compile yet.
- Patterns (constants, variables, tuples, records with punning and `...`, lists, `::`, constructors, layered `as`, type annotations) are parsed, and `case`/`fn`/`handle` matches are checked like SML/NJ does: a redundant rule is an error and a non-exhaustive match is a warning. Or-patterns are an SML/NJ extension and are not accepted.
- Declarations are parsed: `val` with patterns and `and` groups, `val rec`, `fun` with clauses, curried and infix definitions and `and` groups, `local`, `type` abbreviations, and `infix`/`infixr`/`nonfix` (scoped by `let` and `local`). Duplicate variables and function names are rejected as in SML/NJ. `exception` and modules are not parsed yet, and the backend still only compiles simple `val` bindings.
- Types are inferred with Hindley–Milner (`src/infer.rs`) before code generation, and `--dump-types` prints `val name : type` for each top-level binding. It covers polymorphic functions and let-bound values, the value restriction (SML/NJ's `?.X1` dummy types for what cannot be generalised), equality types (`''a`), and overloaded `+ - * < ...` that default to `int`. `tools/update_filecheck.py` cross-checks the printed types against what SML/NJ echoes. Record selectors (`#label`) and flexible record patterns (`...`) are rejected as unsupported, and printed types expand `type` abbreviations where SML/NJ keeps their names. The REPL still uses the older checker.
- `datatype` (with `and`, `withtype`, `op`, infix constructors and replication `datatype t = datatype u`) and `abstype` are parsed and type-checked. Constructors are tracked by scope for pattern checks, so a capitalised name is only a constructor if a datatype declared it, and a match over a datatype is exhaustive exactly when it names every constructor. Each datatype is a distinct type (`t` and a later shadowing `t` print as `t` and `t/2`), datatypes admit equality when their constructors' argument types do, and `abstype` hides its constructors and equality. Replicating a built-in datatype such as `list` is not supported yet.
- `true` and `false`
