# Nassau

A wip sml implementation in rust with a cranelift backend

To inspect Cranelift's compilation stages:

```sh
cargo run -- --debug-passes tests/ret-42.ml
```

This prints the function IR before and after Cranelift optimization, followed by the lowered target instructions.
