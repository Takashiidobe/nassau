# Nassau in the browser

A persistent Standard ML REPL running entirely in a Web Worker. The Rust
frontend lowers SML to core IR and emits Cranelift IR using the same emitter as
the native compiler. The `clif2wasm` bridge translates this through
Waffle into Wasm modules, which the browser compiles and executes locally.
The runtime is the same Rust code the native compiler links
(`runtime/`), built for `wasm32-wasip1`: it supplies allocation, mark-and-sweep
collection, output and exception state, and `frontend/wasi.js` provides the
WASI imports it needs (stdout, exit, environment, clock, random). All editor assets are local; no CDN or execution server is
needed. The browser must support WebAssembly tail calls.

Build from the repository root with Rust, the `wasm32-unknown-unknown` and
`wasm32-wasip1` targets and `wasm-pack` available. Cargo fetches `clif2wasm` from its pinned
[GitHub revision](https://github.com/Takashiidobe/clif2wasm/tree/edf08ece9bd24991c1325cb3e99ee94a026be42d):

```sh
rustup target add wasm32-unknown-unknown wasm32-wasip1
./www/build.sh
python3 www/serve.py 8000
```

Open <http://localhost:8000>. `serve.py` sends the `Cross-Origin-Opener-Policy` and
`Cross-Origin-Embedder-Policy` headers (as `netlify.toml` does in production), which make
the page cross-origin isolated. A program that reads `TextIO.stdIn` then blocks the worker in
`Atomics.wait`, and the page switches to a `stdin>` prompt: Enter sends a line, Ctrl+D sends
the typed text and then end of file. Without isolation, stdin reads return end of file. `./www/build.sh /absolute/output/path` selects a
custom output directory. The build uses `wasm-pack` without wasm-opt and does
not require npm packages. CodeMirror is vendored from the Slate reference
website with its license in `frontend/assets/CODEMIRROR-LICENSE`. The matching
CodeMirror 5.65.16 runMode addon renders submitted input and results using the
same SML syntax mode as the active prompt.

The page is a single REPL. Type into the CodeMirror prompt and submit with
`;`, Ctrl+Enter or Cmd+Enter. `;` submits when typed at the end
of the input, outside a string, comment, bracket or `let`/`struct`/`sig` block; `;;` also works. Enter continues a multiline command. Results appear immediately below that prompt,
followed by a fresh prompt. Submitted input stays in place; it is not copied
into the output. Bindings, fixity, modules, closures
and references remain available between submissions.

The key bindings selector supports Standard, Vim and Emacs and remembers your
choice. Vim supports `:w` and `:submit` to submit a phrase. Ctrl+Up/Ctrl+Down recalls submitted input, preserving your current draft. Tab lists matching names beneath the cursor in columns, completes the highlighted name, and cycles through candidates on repeated presses.
Examples populate the current prompt without resetting the session. `clear;;`
clears previous prompts and results but keeps bindings. `reset;` clears the
display and starts a fresh session. Both show a confirmation at the top of the
terminal: `Cleared` or `Reset`. These commands also work in the native
REPL. They are reserved only when submitted alone;
expressions such as `clear + 1;;` still use ordinary SML bindings. The toolbar
buttons provide the same clear/reset actions. Reset starts a fresh worker. Stop terminates the executing worker and resets the session. Share
links contain the current prompt; opening one starts a fresh session.

Parse and type errors appear beside the submitted source. An uncaught SML
exception discards the failing phrase's new bindings while retaining previous
bindings and effects. Compiler diagnostics refer to the submitted source;
runtime exception locations use the session's cumulative `stdIn` line numbers.
Compiled modules, literals and global cells remain allocated for the session's
lifetime; collection reclaims unreachable heap objects, including cycles.
Closures retain the historical globals their code reads. Reset releases the
whole session, including compiled code. Browsers display printed bytes as UTF-8.

The Wasm API is a `BrowserRepl` with `submit(source)`, returning captured output
bytes, diagnostics, an optional exit status and a display-clear flag. The worker runs this same
frontend session with a Cranelift/Wasm execution backend. Native execution
continues to use Cranelift JIT and the native runtime.

The automated session fixture runs the generated browser Wasm in Node, checks
persistent bindings, exception recovery, direct and indirect tail calls, and
collection under allocation pressure, then compares REPL fixtures with the
native JIT. Build the native executable and web bundle first:

```sh
cargo build --release
./www/build.sh
node tests/fixtures/runtime/browser-session.mjs
```

Browser UI testing is manual. Check loading, example selection, repeated runs,
diagnostics, stop/reset, copy link, and the narrow screen layout. To exercise
Stop, submit `fun loop () = loop (); loop ();` and stop the running session.
