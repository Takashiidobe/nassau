# Browser Cranelift execution

Nassau uses the same core-to-CLIF emitter for native and browser execution.
The browser compiler itself is built with Rust for `wasm32-unknown-unknown`.
For each phrase it emits CLIF, translates through clif2wasm and Waffle, and
instantiates the resulting Wasm in the existing Web Worker. Execution needs
no native REPL process or compiler service.

The `codegen` feature contains the common emitter. `native` adds the JIT,
object emitter and MMTk runtime; `web` adds clif2wasm and the browser bindings.
Cranelift 0.136.0 is shared with clif2wasm. Pulley32 supplies the bridge's
32-bit target configuration; it does not execute SML or compile native code.
The browser path consumes CLIF before ISA-specific machine-code compilation.

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

`www/frontend/runtime.js` owns the session memory and runtime imports. The
shadow stack occupies the first megabyte above the null guard; statics and
heap objects grow above it. The translator checks shadow-stack bounds before
writing a frame. Proper tail calls restore the shadow stack before transfer.
Waffle emits marker calls that a Wasm reencoder replaces with `return_call`
or `return_call_indirect`; ordinary calls remain ordinary calls.

The runtime uses a nonmoving mark-and-sweep collector with a coalescing free
list. Roots include active CLIF root frames, visible globals, exception state,
builtin exception identities and host-side value formatting. Live closures
also trace the historical globals their functions depend on. Strings and
reals do not contain traced fields; closure code indices are not heap pointers.
Static literals, global cells and compiled instances remain session-owned
until reset. Output is captured as bytes. SML exceptions follow the existing
status protocol; Wasm traps and unexpected host errors stop the worker.

## Dependency source

Nassau pins [clif2wasm](https://github.com/Takashiidobe/clif2wasm) at
`edf08ece9bd24991c1325cb3e99ee94a026be42d`, with Cranelift 0.136.0 and
Waffle 0.3.2. No sibling checkout or vendored snapshot is required.

The former core-IR interpreter and native `--interpret` mode are removed.
Browser execution uses generated Wasm; native REPL execution uses Cranelift
JIT, and native file compilation uses Cranelift's object backend. The shared
session, lowering, CLIF emitter and value printer serve both REPL hosts.
