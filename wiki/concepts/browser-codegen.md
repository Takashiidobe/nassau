# Browser Cranelift execution

Nassau uses the same core-to-CLIF emitter for native and browser execution.
The browser compiler itself is built with Rust for `wasm32-unknown-unknown`.
For each phrase it emits CLIF, translates through clif2wasm and Waffle, and
instantiates the resulting Wasm in the existing Web Worker. Execution needs
no native REPL process or compiler service.

The `codegen` feature contains the common emitter. `native` adds the JIT,
object emitter and native runtime; `web` adds clif2wasm and the browser bindings.
Cranelift 0.136.0 is shared with clif2wasm. Its x64 ISA only satisfies
Cranelift's `Module` interface; `WasmModule::target_config()` supplies Wasm's
32-bit pointers and System V convention. The browser path consumes CLIF before
ISA-specific machine-code compilation.

## Values and state

Tagged SML values and object fields remain 64-bit words. Actual memory
addresses and closure function-table indices are 32-bit values. The emitter
converts at loads, stores, symbol and stack addresses, indirect calls and
memory helper boundaries. This preserves the existing eight-byte layout and
31-bit SML integer semantics.

`WasmModule::emit_incremental` gives each data object a stable session address,
initializes only new data, and imports earlier functions by name. Every chunk
imports the session's memory, growable function table and stack-pointer global.
Function IDs determine stable table slots. Earlier instances remain reachable
through their function exports; saved closures keep calling their original
code after names are shadowed. Compiler declarations and symbols are staged
until the generated module successfully instantiates.

## Runtime

The runtime is `runtime/`, the same Rust code native builds link, compiled for
`wasm32-wasip1` as a reactor (`nassau_runtime.wasm`). It is linked with
`--global-base=1114112 --no-stack-first` so its own data and stack sit above
the 1 MiB shadow stack that clif2wasm hard-codes at `[4096, 1MiB)`. It owns
linear memory: generated modules import its `memory` export, and static data
comes from `nassau_static_alloc`. Every export takes and returns 64-bit words,
matching the emitter's helper signatures, so the collector reads root frames
and object fields as `u64` rather than `usize`.

`www/frontend/runtime.js` instantiates it, owns the function table and the
stack-pointer global, and maps the generated modules' `env` imports onto its
exports. `www/frontend/wasi.js` supplies
the WASI imports. Output written to fd 1 and 2 is captured as bytes, `proc_exit`
becomes the session's exit status, and `NASSAU_GC_*` settings arrive through the
WASI environment. The collector, roots, exceptions and `Posix.Process.exit`
behave exactly as on native. Wasm traps and unexpected host errors stop the
worker.

## Dependency source

Nassau pins [clif2wasm](https://github.com/Takashiidobe/clif2wasm) at
`43274bf90f7070da3221f6de924e9bfc11a284d9`, with Cranelift 0.136.0 and
Waffle. The pin includes the x64 frontend facade and
`WasmModule::target_config()`; the module seam is on mainline and needs no
feature flag.

The former core-IR interpreter and native `--interpret` mode are removed.
Browser execution uses generated Wasm; native REPL execution uses Cranelift
JIT, and native file compilation uses Cranelift's object backend. The shared
session, lowering, CLIF emitter and value printer serve both REPL hosts.

## OS services

Basis functions that reach the OS are table-driven externs. `EXTERN_FNS` in
`src/core.rs` lists each as `Prim.<name>`, its C symbol, parameter types
(`Int`, `String`, `Real`, `Unit`) and result type; one `Prim::Extern` serves
them all. The `nassau_math_*` functions use it (`sqrt` is inline), as do the `nassau_os_*`
entry points in `runtime/os.rs`: file open/read/write/close/remove,
`getenv` and error messages. `runtime/os.rs` uses plain `std::fs` and
`std::env`, so the same code runs natively and, built for `wasm32-wasip1`,
in the browser.

In the browser `www/frontend/wasi.js` implements the WASI imports over an
in-memory filesystem preopened at `/`: open, read, write, seek, stat, unlink and
directories. Output on fd 1 and 2 is captured; the environment starts empty.
The `Wasi` constructor takes the environment and initial files. Adding an OS
service means a row in `EXTERN_FNS`, a function in `runtime/os.rs`, and an entry
in `runtime::symbols()` for the JIT; the browser picks up every `nassau_*`
export automatically. Native paths and the browser's virtual files are covered
by `tests/fixtures/basis/textio-files.sml`, which `browser-session.mjs` also runs.
