# Runtime value representation

Every SML value is one 64-bit word. The low bit says what it is: 1 for an
immediate, 0 for an 8-byte-aligned pointer to a heap block. `src/value.rs` holds
the constants shared by code generation and the runtime.

## Immediates

An immediate stores `n` as `(n << 1) | 1`.

| Type                                     | `n`                                                 |
| ---------------------------------------- | --------------------------------------------------- |
| `int`                                    | the integer                                         |
| `char`                                   | the character code                                  |
| `bool`                                   | `false` = 0, `true` = 1                             |
| `unit`                                   | 0                                                   |
| nullary constructor (`nil`, `NONE`, ...) | its index among the datatype's nullary constructors |

`nil`, `false`, `()` and `0` are all the word 1. Types keep them apart.

## Heap blocks

A pointer addresses the block's header word. Field `i` is at byte offset
`8 * (i + 1)`.

```
header = (length << 8) | kind
```

| Kind    | Value | Length counts | Contents                                          |
| ------- | ----- | ------------- | ------------------------------------------------- |
| record  | 0     | fields        | tuples, records, cons cells, constructor payloads |
| closure | 1     | fields        | field 0 is the code address, then free variables  |
| string  | 2     | bytes         | NUL-terminated bytes, padded to a word            |
| real    | 3     | 1             | one IEEE double                                   |
| ref     | 4     | 1             | one mutable value                                 |
| array   | 5     | elements      | mutable SML values                                |
| vector  | 6     | elements      | immutable SML values                              |

The collector traces record, array, vector, and ref fields and closure fields
after field 0. String and real blocks hold raw bytes.

## Layout by construct

- **Tuples and records:** a record block with one field per component, in label
  order (`#1`, `#2`, ... for tuples). `()` is an immediate, not a block.
- **Lists:** cons cells are two-field records (head, tail) ending in `nil`.
- **Constructors with an argument:** a record block. If the datatype has several
  such constructors, field 0 is the constructor's index among them (an
  immediate) and field 1 is the argument. If it has one, like `SOME`, the block
  holds only the argument.
- **Reals:** boxed. Code generation keeps them unboxed in registers inside a
  function.
- **Strings:** string blocks. The NUL terminator lets the runtime pass them to
  C.
- **Closures:** the code address plus captured variables. A call passes the
  closure itself as the environment argument.

## Exceptions

An exception is a record `[identity, argument, raised at]`.

- `argument` is `()` for a constructor that takes none.
- `raised at` is `()` until the first raise, then a string naming the location.
  Re-raising keeps it.
- `identity` is a ref cell holding the exception's name, allocated each time the
  `exception` declaration is evaluated. Declarations inside functions are
  therefore generative. A replication (`exception F = E`) shares the original's
  identity. A match compares identities by address.

In the REPL, the identity is instead `[name, payload type descriptor]`, with
`()` as the descriptor for nullary exceptions. A descriptor is a record:

- tag 0: type name, stamp, and a record of argument descriptors
- tag 1: a record of `[label, descriptor]` fields
- tags 2 and 3: functions and unknown variables

Raising does not unwind the stack. A function that raises records the exception
with the runtime and returns the word `0`, which is not a valid value. Every
call checks for `0` and either enters the enclosing handler or returns `0`
itself. A call under a handler is therefore not a tail call.

## `int`

`int` is 31 bits: `Int.minInt` is `~1073741824` and `Int.maxInt` is
`1073741823`, matching 32-bit SML/NJ. The fixture harness compares integer-limit
behavior with the oracle only when its `Int.precision` matches.

A constant outside the range is a compile-time error (`int constant too large`).
Arithmetic that leaves the range raises `Overflow`, and `div` or `mod` by zero
raises `Div`.
