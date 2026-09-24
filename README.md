# Nassau

A work-in-progress Standard ML implementation in Rust with a Cranelift backend.

## Features

- Native compilation with Cranelift.
- Type inference with polymorphism, equality types, and the value restriction.
- Functions, closures, recursion, and tail calls.
- Tuples, records, lists, and pattern matching.
- Datatypes, exceptions, references, and mutable state.
- Structures, signatures, and functors.
- Persistent REPL with value printing and declaration echoes.
- Compiler diagnostics, IR dumps, and assembly inspection.
- End-to-end fixtures checked against Poly/ML and FileCheck.

## Usage

- Start the REPL: `cargo run`.
- Start the interpreter REPL: `cargo run -- --interpret`.
- The native REPL saves submitted phrases in the platform data directory and completes in-scope names with Tab.
- In either REPL, `clear;;` clears the display; `reset;;` clears it and discards bindings.
- Compile a program: `cargo run -- input.sml`.
- Interpret a program with the portable Rust backend: `cargo run -- --interpret input.sml`.
- Run fixtures: `cargo test` (requires `poly` and `FileCheck`).

## Documentation

- [Language support and grammar](docs/grammar.md)
- [Compiler architecture](docs/architecture.md)
- [Value representation](docs/value-representation.md)
- [Development and fixture guidance](docs/development.md)

The [browser playground](www/README.md) runs a persistent SML REPL entirely
on the client. Build with `./www/build.sh`, then serve `www/dist` with a static
HTTP server.
