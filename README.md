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

## Features

- Values share one word-sized representation (`docs/value-representation.md`): `int` is 31 bits wide as in SML/NJ, and lists are cons cells.
- Code generation lowers the type-checked program to a core IR in A-normal form with explicit blocks, then translates that to Cranelift. The REPL compiles each chunk the same way and prints bindings from their compiled values.
- A C runtime library (`runtime/nassau_runtime.c`, built by `build.rs`) is linked into compiled programs and into the compiler for the REPL's JIT: allocation, printing, string concatenation, structural equality and uncaught exceptions, which it reports with SML/NJ's message and exit status.
- Top-level `fun` and `val rec` compile to native functions, including mutual recursion with `and` and clauses over constants, tuples, records, lists and `::`; calls to a known function go straight to its code. `val` bindings take any of those patterns, raising `Bind` when they fail, and a match with no applicable rule raises `Match`. `print`, `Int.toString`, `^`, `size`, `not` and `~` are compiled inline.
- `fn` expressions and functions returned from functions are closures: closure conversion gives each one a code pointer and the variables it captures, and calls to unknown functions go through the closure, so functions can be passed, returned and stored in data. `o`, references (`ref`, `!`, `:=` and `ref` patterns), `while`, `before`, `ignore`, `#label` selectors and functions declared infix are compiled too.
- Curried functions (`fun f x y = ...`) compile to a worker taking every argument plus one small function per stage, so a partial application allocates a closure holding the arguments so far, while a saturated call such as `f 1 2` goes straight to the worker without allocating.
- Calls in tail position are proper tail calls (Cranelift's tail calling convention and `return_call`), including mutual recursion and calls through closures, so loops written as recursion run in constant stack in compiled programs and the REPL.
- Polymorphic functions are compiled once and used at every type through the uniform representation (reals, strings, tuples and lists are passed as pointers); `=` on an equality type variable (`''a`), strings, lists and tuples uses the runtime's structural equality.
- Integer arithmetic supports `+`, `-`, `*`, and `div` (rounding toward negative infinity), raising `Overflow` outside the 31-bit range and `Div` on division by zero.
- Real arithmetic supports `+`, `-`, `*`, and `/`.
- Conditionals like `If then else`
- Relops, CmpOps like `<=, >=, <, >, =, <>`
- Basic list construction
- The parser accepts the full SML expression syntax (application, default infix fixities, tuples, records, selectors, sequences, `let`, `case`, `fn`, `while`, `raise`/`handle`, type annotations); `--dump-ast` prints the tree. Code generation still rejects the forms it cannot compile yet.
- Patterns (constants, variables, tuples, records with punning and `...`, lists, `::`, constructors, layered `as`, type annotations) are parsed, and `case`/`fn`/`handle` matches are checked like SML/NJ does: a redundant rule is an error and a non-exhaustive match is a warning. Or-patterns are an SML/NJ extension and are not accepted.
- Declarations are parsed: `val` with patterns and `and` groups, `val rec`, `fun` with clauses, curried and infix definitions and `and` groups, `local`, `type` abbreviations, and `infix`/`infixr`/`nonfix` (scoped by `let` and `local`). Duplicate variables and function names are rejected as in SML/NJ. modules are not parsed yet, and the backend still only compiles simple `val` bindings.
- Types are inferred with Hindley–Milner (`src/infer.rs`) before code generation, and `--dump-types` prints `val name : type` for each top-level binding. It covers polymorphic functions and let-bound values, the value restriction (SML/NJ's `?.X1` dummy types for what cannot be generalised), equality types (`''a`), and overloaded `+ - * < ...` that default to `int`. `tools/update_filecheck.py` cross-checks the printed types against what SML/NJ echoes. Record selectors (`#label`) and flexible record patterns (`...`) are rejected as unsupported, and printed types expand `type` abbreviations where SML/NJ keeps their names. The compiler and REPL use this checker only; code generation reports the constructs it cannot compile yet.
- `datatype` (with `and`, `withtype`, `op`, infix constructors and replication `datatype t = datatype u`) and `abstype` are parsed and type-checked. Constructors are tracked by scope for pattern checks, so a capitalised name is only a constructor if a datatype declared it, and a match over a datatype is exhaustive exactly when it names every constructor. Each datatype is a distinct type (`t` and a later shadowing `t` print as `t` and `t/2`), datatypes admit equality when their constructors' argument types do, and `abstype` hides its constructors and equality. Replicating a built-in datatype such as `list` is not supported yet.
- `exception` declarations (`exception E`, `exception E of ty`, `and` groups, and replication `exception F = E`) are parsed and type-checked, including in `let` and `local`. Exception constructors have type `exn`, are scoped like other constructors, and form an open family, so a `handle` or `fn` over them is redundant only when a rule repeats one and a non-exhaustive `fn`/`case` over `exn` warns as in SML/NJ. `raise` requires an `exn` and `handle` arms must agree with the body's type. Only the front end handles exceptions so far; the backend still rejects them as unsupported.
- Modules are parsed and type-checked in the front end: `structure` (with `and`, `let`, nested structures and structure aliases), `signature` (`val`, `type`, `eqtype`, `datatype`, datatype replication, `exception`, `structure`, `include` and `sharing type` specifications), `where type` refinement, `open`, and qualified names (`S.x`, `S.t`, `S.C` in expressions, patterns and types). A structure matches a signature when every specification is met by a definition at least as general; `S : SIG` (transparent) keeps the identity of the structure's types, while `S :> SIG` (opaque) gives each abstract type and datatype a fresh type, so `S.t` printed as `S.t` cannot be seen through and equality needs `eqtype`. Constructors hidden by a signature are hidden from patterns too. The backend and REPL still reject module declarations.
- Functors are parsed and type-checked in the front end: `functor F (X : SIG) = strexp`, the derived form `functor F (specs)` with applications `F (decs)`, `and` groups, and transparent or opaque result signatures. A functor's body is checked once against its parameter signature, so it cannot rely on what an abstract parameter type really is; each application matches the argument against the parameter signature and elaborates the body again, so datatypes and opaque result types are new at every application (`F (A).t` and `F (B).t` differ). Signatures can also share whole structures (`sharing A = B`), which shares every type both specify. As in SML'97, functors and signatures are declared only at the top level or inside a top-level `local`.
- References and imperative expressions are type-checked: `ref`, `!` and `:=` (`'a -> 'a ref`, `'a ref -> 'a`, `'a ref * 'a -> unit`), `before`, sequences and `while`. `ref` is a constructor, so `ref p` patterns work and are exhaustive on their own, but `ref e` is never generalised, so `val r = ref []` keeps a dummy `?.X1` type as in SML/NJ. The backend does not run references yet.
- `true` and `false`
