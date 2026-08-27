# Nassau in the browser

A persistent Standard ML REPL running entirely in a Web Worker. The Rust
frontend lowers SML to core IR, which the portable Rust interpreter executes.
Boa GC manages values. Cranelift and the native MMTk runtime are excluded from
this build. All editor assets are local; no CDN or execution server is needed.

Build from the repository root with Rust, the `wasm32-unknown-unknown` target
and `wasm-pack` available:

```sh
rustup target add wasm32-unknown-unknown
./www/build.sh
python3 -m http.server 8000 --directory www/dist
```

Open <http://localhost:8000>. `./www/build.sh /absolute/output/path` selects a
custom output directory. The build uses `wasm-pack` without wasm-opt and does
not require npm packages. CodeMirror is vendored from the Slate reference
website with its license in `frontend/assets/CODEMIRROR-LICENSE`. The matching
CodeMirror 5.65.16 runMode addon renders submitted input and results using the
same SML syntax mode as the active prompt.

The page is a single REPL. Type into the CodeMirror prompt and submit with
`;;`, Ctrl+Enter or Cmd+Enter. `;;` submits when typed at the end
of the input, outside a string or comment. Enter continues a multiline command. Results appear immediately below that prompt,
followed by a fresh prompt. Submitted input stays in place; it is not copied
into the output. End internal phrases with single semicolons; `;;` finishes a submission. Bindings, fixity, modules, closures
and references remain available between submissions.

The key bindings selector supports Standard, Vim and Emacs and remembers your
choice. Vim supports `:w` and `:submit` to submit a phrase. Ctrl+Up/Ctrl+Down recalls submitted input, preserving your current draft.
Examples populate the current prompt without resetting the session. `clear;;`
clears previous prompts and results but keeps bindings. `reset;;` clears the
display and starts a fresh session. Both show a confirmation at the top of the
terminal: `Cleared` or `Reset`. These commands also work in the native
REPL, including `--interpret`. They are reserved only when submitted alone;
expressions such as `clear + 1;;` still use ordinary SML bindings. The toolbar
buttons provide the same clear/reset actions. Reset starts a fresh worker. Stop terminates the executing worker and resets the session. Share
links contain the current prompt; opening one starts a fresh session.

Parse and type errors appear beside the submitted source. An uncaught SML
exception discards the failing phrase's new bindings while retaining previous
bindings and effects. Compiler diagnostics refer to the submitted source;
runtime exception locations use the session's cumulative `stdIn` line numbers.
The interpreter retains function IR for the session's lifetime, so reset also
releases historical code. Browsers display printed bytes as UTF-8.

The Wasm API is a `BrowserRepl` with `submit(source)`, returning captured output
bytes, diagnostics, an optional exit status and a display-clear flag. The worker runs this same
persistent session used by `nassau --interpret`.

Browser UI testing is manual. Check loading, example selection, repeated runs,
diagnostics, stop/reset, copy link, and the narrow screen layout. To exercise
Stop, submit `fun loop () = loop (); loop ();` and stop the running session.
