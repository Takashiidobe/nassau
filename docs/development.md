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

Earlier stages have their own dumps: `--dump-expr-types` prints the inferred type of every expression and pattern, and `--dump-core` prints the core IR a program lowers to before Cranelift (`src/core.rs`). The dump leaves out the SML basis in `basis/`, so it cannot show a program that uses a basis structure such as `List`.

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

The `interpreter/<fixture>` trials execute accepted programs with `--interpret`
and check the same runtime expectations and Poly/ML oracle. Native MMTk
fixtures and constructs rejected by shared lowering remain excluded. The
standalone session fixture checks historical closure globals and reclamation
of unreachable cycles with `boa_gc` weak references:

```sh
cargo test --test fixtures interpreter/
cargo test --test interpreter-session
```

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
regeneration of runtime checks. The five remaining exclusions are tracked in
`nassau-949.12`: infix datatype constructors, `~` negation, and word
arithmetic.

`tests/fixtures/grammar` holds one fixture per production of the SML'97
grammar. Productions Nassau does not handle yet live in
`tests/fixtures/grammar.unsupported`, where each header names the tracking
bead. Poly/ML must accept them and Nassau must still fail (by rejecting them,
or by producing different output natively or in the interpreter). Once one
passes, its trial fails and prints the `git mv` that moves it into
`tests/fixtures/grammar`. The same applies to any `*.unsupported/` directory.

External suites are imported with `tools/import_suite.py <suite> --source <checkout>`
at a pinned upstream commit, with Poly/ML as the only oracle. Each suite keeps its
own licence and `PROVENANCE` in `tests/fixtures/<suite>/`, and every file's first
line names its upstream path and commit. Programs land in `<suite>/` (agree),
`<suite>/error/` (both reject), `<suite>.unsupported/` (Poly/ML accepts, Nassau
does not match) and `<suite>.unsupported/error/` (Poly/ML rejects, Nassau
accepts), or `<suite>.ignored/` where Poly/ML itself disagrees with the suite
(line 2 says why; the harness never runs these). `<suite>/EXCLUDED.tsv` lists
programs skipped for MLton primitives, FFI imports or `use`. Re-running the tool
on the pinned commit reproduces the layout.

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

## MMTk runtime

`runtime` is a Cargo workspace crate, shared as an rlib by the compiler/JIT
and as a staticlib by generated native programs. MMTk is pinned to 0.33.0
with default features disabled and `vo_bit` enabled (which enables eager
sweeping). The runtime requires `std`; it no longer provides a separate
`no_std` allocator or panic handler. The native archive is built in a separate
Cargo target directory with `panic=abort`, includes its Rust dependencies, and
is embedded in the compiler. The JIT links only the rlib. `build.rs` obtains
required system libraries from rustc's `--print=native-static-libs` output;
native linking still honors `NASSAU_CC`. Build errors retain Cargo diagnostics.
The tested platform is x86_64 Linux with rustc 1.100.0-nightly
(5a2be9f5f, 2026-09-06); other platforms are not verified.

The default production plan is non-moving MarkSweep, using a 32 MiB fixed
heap and one worker. `NASSAU_GC_PLAN=NoGC` selects the allocation-only plan;
`NASSAU_GC_HEAP=8m` changes the heap size. `NASSAU_GC_STRESS=N` requests
a collection before every Nth allocation (0 disables forced collections).
These are Nassau options; other MMTk environment options are not imported.
A process has one lazily initialized MMTk instance and a thread-local bound
mutator, shared by successive REPL chunks. Thread exit flushes and destroys
the mutator. SML runtime globals assume one SML execution thread. Exhaustion
prints `nassau: out of memory` and exits with status 1. NoGC proves allocation
and linkage, not bounded memory.

The allocation ABI is `nassau_alloc(length, kind)`, where length counts bytes
for strings and fields for other kinds. It checks header representability,
physical size, and object kind before allocating at 8-byte alignment. Allocations
above the plan's default-allocator limit use large-object semantics for both
`alloc` and `post_alloc`. A string
uses `8 + 8 * (length / 8 + 1)` bytes, including its terminator and padding;
other objects use `8 * (length + 1)`. Before `post_alloc`, the runtime writes
the header and zeros the entire payload. Generated code then fills fields.
There is no safepoint during these stores; nested allocations require the
in-progress object and allocation operands to be rooted. The generated shadow
stack and runtime scopes provide these roots (`nassau-949.9.4`). Zeroed fields
remain non-reference sentinels if an object is scanned while nested
initialization is in progress.

The object reference is the aligned header address. Mark bits, forwarding
status and large-object mark/nursery bits use MMTk side metadata. The forwarding
pointer specification reserves the header only for unsupported moving plans;
MarkSweep does not overwrite it. Object scanning visits records and refs,
and closure captures after the raw code field. Strings and reals have no
reference fields. Tagged slots reject zero, misaligned words, static data and
addresses without valid-object metadata before constructing object references;
valid-object bits remain available during tracing. Writable slots use atomic
word loads and stores. This does not enable moving collectors.

Static language objects emitted by `Translator::data` are exclusively
string and real literals, so they contain no managed children. Writable JIT
globals are separate root slots, not static language objects; their lifetime
management is tracked in `nassau-949.9.5`. Exception objects and REPL type
descriptors are ordinary managed record/ref graphs and follow those scanning
rules. Nassau has no weak-reference or finalizer objects.

Run allocation and object-model fixtures with:

```sh
cargo test --test fixtures mmtk-nogc
cargo test --test repl
cargo test -p nassau-runtime --test object-model
```

The object-model fixture checks size/alignment, initialized payloads, rejected
slots and precise field visitation, then launches a separate process with a
controlled MarkSweep binding using the production object model and scanner.
Two forced collections retain shared cyclic records/refs/closure captures,
reclaim unreachable cycles, and reject pointer-shaped closure code, real and
string payloads. This fixture supplies explicit roots and mutator coordination;
its graph checks complement production native/REPL collection fixtures.
The native collecting fixture allocates a million iterations of unreachable
cycles under an 8 MiB heap while preserving a shared live graph. The REPL
history fixture shadows 40 large strings under that heap and verifies that
a saved closure still accesses its earlier global after collection. The functor
history fixture retains forty functors while shadowing large strings, then checks
original structure and exception dependencies through a forwarding functor.

Generated functions publish precise shadow-stack frames. A backward fixed-point
analysis computes tagged variables live across each statement and terminator,
including handler paths. Publication clears stale slots and spills the live
values; allocated objects and runtime-call results occupy temporary slots until
the next statement. Closure groups therefore root their members during mutual
initialization. Helpers root their arguments, and all normal/exception returns
and tail transfers pop their frame. A tail transfer has no allocation between
popping the caller and publishing the callee frame. The collector runs only
while the single SML mutator is parked. This fallback avoids a platform-specific
frame walker and native/JIT stack-map relocation requirements.

Runtime `with_roots` scopes use the same linked-frame layout. Built-in exception
construction roots its partially filled ref across string allocation; REPL
value/uncaught printing roots host-held objects across built-in exception
initialization. The collector scans exception identities and raised/uncaught
state in addition to published frames. The isolated fixture forces collection
with objects reachable exclusively through nested stack and host scopes, then
verifies reclamation after those scopes are removed.

Generated entry functions register their module's global cells and function
dependencies before allocation. Each function's registered dependencies include
its global reads/stores and the transitive dependencies of statically called
functions and created closures. A frame records its code address; root scanning
visits the corresponding global cells. When scanning a live closure, its raw
code address selects the same dependency metadata without treating the address
as a heap reference. This keeps globals used by old callable code alive without
rooting every historical module. JIT code and dependency metadata are retained
for the executable code's lifetime; code unloading remains outside the scope.

After a successful REPL phrase and its printing, persistent roots become the
visible globals, visible structure exports and captured dependencies
of visible functors. A failed phrase restores the earlier environment before
updating roots. Functor roots select declaration-time bindings whose names appear
in the body,
including exception patterns and replications, referenced structures and the
transitive roots of applied functors. Unmentioned lexical bindings are excluded.
This syntactic analysis may retain a matching outer name that the body shadows,
and retains all exports of a referenced structure. The compiler still keeps the
full lexical metadata for later elaboration; it does not root its heap values.
Native global registrations last for the executable's lifetime.

Fixture comments `GC-PLAN`, `GC-HEAP`, and `GC-STRESS` set the corresponding
runtime options only for Nassau execution. Poly/ML still checks the same SML
behavior. `mmtk-nogc.sml` explicitly exercises NoGC; `mmtk-collecting.sml`,
`mmtk-history.sml` and `mmtk-exhaustion.sml` exercise production collection and
configured exhaustion. Run the complete suite with periodic forced collection using:

```sh
NASSAU_GC_STRESS=1000 NASSAU_GC_HEAP=32m cargo test --workspace
```

The short `mmtk-safepoints.sml` fixtures force collection at every allocation.
Using that interval for growing long-list fixtures makes GC work quadratic;
the full suite uses a larger interval and a heap large enough for its live
graphs. The collecting/history fixtures independently enforce their 8 MiB heap.

The REPL fixture harness runs transcripts through both the native JIT and
`--interpret`; native GC-plan fixtures remain specific to the JIT. The standalone
`interpreter-session` fixture also checks separate submissions, error recovery,
exception effects, process exit, reset and historical closure collection.

REPL fixtures marked `(* REPL-COMMANDS *)` exercise Nassau host commands
without a Poly/ML comparison. They check terminal clearing and fresh bindings
after reset against both backends.
