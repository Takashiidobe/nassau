# Runtime value representation

Every SML value is one 64-bit machine word, so polymorphic functions,
closures and data structures can hold any value without knowing its type.
A word is either an **immediate** (low bit 1) or a **pointer** to a heap block
(low bit 0, 8-byte aligned). `src/value.rs` holds the constants code
generation and the runtime share.

## Immediates

An immediate stores a small integer `n` as `(n << 1) | 1`.

| Type                                     | Value of `n`                                                      |
| ---------------------------------------- | ----------------------------------------------------------------- |
| `int`                                    | the integer itself                                                |
| `char`                                   | the character code                                                |
| `bool`                                   | `false` = 0, `true` = 1                                           |
| `unit`                                   | 0                                                                 |
| nullary constructor (`nil`, `NONE`, ...) | the constructor's index among the datatype's nullary constructors |

`nil` is therefore the word 1, the same word as `false`, `()` and `0`; types
keep them apart.

## Heap blocks

A pointer addresses a block's **header word**; field `i` is at byte offset
`8 * (i + 1)`. The header holds the block's length above a kind byte, so a
later garbage collector can walk the heap without type information:

```
header = (length << 8) | kind
```

| Kind    | Value | Length counts | Contents                                                       |
| ------- | ----- | ------------- | -------------------------------------------------------------- |
| record  | 0     | fields        | values: tuples, records, cons cells, constructor payloads      |
| closure | 1     | fields        | field 0 is a code address, the rest are free variables' values |
| string  | 2     | bytes         | the bytes, NUL-terminated and padded to a word                 |
| real    | 3     | 1             | one IEEE double, not a value                                   |
| ref     | 4     | 1             | one mutable value                                              |

Values of the record and ref kinds and closures after field 0 are values the
collector must trace; string and real blocks hold raw bytes.

- **Tuples and records** are record blocks with one field per component, in
  label order (`#1`, `#2`, ... for tuples). `()` is the immediate 1, not a
  block.
- **Lists** are chains of cons cells ending in `nil`: a cons cell is a record
  block with two fields, the head and the tail.
- **Constructors with an argument** are record blocks. When the datatype has
  several such constructors, field 0 is the constructor's tag (an immediate:
  its index among them, in declaration order) and field 1 its argument. When
  it has only one, as `option` has `SOME`, the tag is left out and the block
  holds just the argument; a match tells it from the nullary constructors,
  which are immediates, by whether the value is a block.
- **Exceptions** are record blocks `[identity, argument, raised at]`, with
  `()` as the argument of a constructor that takes none. The last field is
  `()` until the value is first raised and then the string naming where, as
  SML/NJ reports it; raising the same value again keeps it. The identity is a reference
  cell holding the exception's name, allocated each time the `exception`
  declaration is evaluated, so declarations inside functions are
  generative; a replication (`exception F = E`) shares its original's
  identity. A match compares identities by address.
- **Raising** does not unwind the stack by jumping. A function that raises
  records the exception with the runtime and returns the word `0`, which is
  no value: immediates are odd and blocks are non-null. Every call checks
  its result for `0` and either enters the handler of the `handle`
  expression around it, which takes the exception from the runtime, or
  returns `0` itself. Calls under a handler are not tail calls, since the
  handler must outlive them; any other call in tail position still is.
- **Reals** are boxed, so a `real` is a pointer to a real block; code
  generation keeps them unboxed in registers inside a function.
- **Strings** are string blocks. The NUL terminator lets the runtime pass them
  to C directly.
- **Closures** hold the code address and the captured variables; calling one
  passes the closure itself as the environment argument.

## `int`

`int` is 31 bits wide: `Int.minInt` is `~1073741824` and `Int.maxInt` is
`1073741823`, matching 32-bit SML/NJ. The fixture harness checks SML/NJ's
`Int.precision` before comparing integer-limit behavior; 64-bit SML/NJ uses
63-bit integers. MLton uses 32 bits. Tagging leaves 63 bits, so any 31-bit width fits, and
matching SML/NJ keeps oracle fixtures that print or overflow at the limits
comparable.

A constant outside that range is a compile-time error (`int constant too
large`, SML/NJ's wording). Arithmetic that leaves the range raises
`Overflow`, and `div`/`mod` by zero raise `Div`.
