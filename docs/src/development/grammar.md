# Language support

Nassau compiles core SML (functions, data, pattern matching, references,
exceptions) and modules (structures, signatures, functors). This page lists what
works today. The grammar sketches show accepted forms, not the full SML
Definition grammar. See [architecture.md](architecture.md) for how these forms
move through the compiler.

## What works where

"Native" means compiling a file to an executable. The REPL shares its lowering
and code generation.

| Form                                                 | Checker | Native / REPL                 |
| ---------------------------------------------------- | ------- | ----------------------------- |
| Integers, reals, strings, characters, booleans, unit | Yes     | Yes                           |
| Word literals and equality                           | Yes     | Supported word constants only |
| Word arithmetic and ordering                         | Yes     | No                            |
| Tuples, records, lists, selectors                    | Yes     | Yes                           |
| Application, `fn`, `fun`, recursion, currying        | Yes     | Yes                           |
| `if`, `case`, `andalso`, `orelse`, sequences         | Yes     | Yes                           |
| `val`, `let`, `local`, type annotations and aliases  | Yes     | Yes                           |
| `datatype`, `withtype`, `abstype`, replication       | Yes     | Yes                           |
| Exceptions, replication, `raise`, `handle`           | Yes     | Yes                           |
| References, assignment, `while`, `before`            | Yes     | Yes                           |
| Fixity declarations and user infix functions         | Yes     | Yes                           |
| Structures, signatures, `open`, qualified names      | Yes     | Yes                           |
| Functors and sharing                                 | Yes     | Yes                           |
| Standard Basis Library                               | Subset  | Smaller subset                |

## Tokens and literals

- Names are alphanumeric (`count`, `value'`) or symbolic (`++`, `!`).
- Type variables start with `'a`; equality type variables with `''a`.
- Comments are `(* ... *)` and nest.
- Strings use `"..."` and characters use `#"..."`. All standard escapes work,
  including `\ddd`, `\^A`, and `\ ... \` gaps.

| Literal   | Examples                | Notes                                  |
| --------- | ----------------------- | -------------------------------------- |
| Integer   | `42`, `~42`, `0x2a`     | 31 bits: `~1073741824` to `1073741823` |
| Real      | `1.5`, `1e3`, `~1.2e~3` | IEEE 64-bit                            |
| Word      | `0w42`, `0wx2a`         | Constants, matching, and equality only |
| String    | `"hello"`, `"a\000b"`   |                                        |
| Character | `#"a"`, `#"\n"`         |                                        |
| Boolean   | `true`, `false`         |                                        |
| Unit      | `()`                    | Same as `{}`                           |

Integers outside the 31-bit range are rejected, and there are no
arbitrary-precision integers. The uppercase `0wX` spelling is not handled.

A file holds declarations, optionally separated by semicolons. A file containing
only an integer is a legacy shortcut for the exit status. The REPL takes
semicolon-terminated declarations and expressions; an expression binds to `it`.

## Expressions

`e` is an expression, `p` a pattern, `t` a type.

```text
e ::= literal | name | op name
    | () | (e) | (e, e, ...) | {label = e, ...} | [e, ...]
    | #label | e e | e infix-name e | e : t
    | if e then e else e
    | e andalso e | e orelse e
    | (e; e; ...) | let declarations in e; ... end
    | fn rules | case e of rules
    | while e do e
    | raise e | e handle rules

rules ::= p => e [| p => e ...]
```

Functions can capture locals, be returned, and be stored in data. Tail calls,
including mutual recursion and calls through closures, don't grow the stack.

- Integer `+ - * ~ div mod` and real `+ - * ~ /` are compiled.
- `<`, `<=`, `>`, `>=` work on integers and reals.
- Integer arithmetic raises `Overflow` outside the 31-bit range, and `div` or
  `mod` by zero raises `Div`.
- `=` and `<>` work on equality types. Compound values compare structurally and
  references compare by identity. Functions and reals are not equality types.
- `andalso` and `orelse` short-circuit.
- `while` needs a boolean condition and a unit body.

Initial infix declarations (higher binds tighter):

| Precedence | Operators        | Associativity |
| ---------- | ---------------- | ------------- |
| 7          | `* / div mod`    | Left          |
| 6          | `+ - ^`          | Left          |
| 5          | `:: @`           | Right         |
| 4          | `= <> > >= < <=` | Left          |
| 3          | `:= o`           | Left          |
| 0          | `before`         | Left          |

Being in this table doesn't mean an operator is implemented. Check the
[Basis subset](#the-basis-subset).

## Patterns

```text
p ::= _ | name | op name | non-real literal
    | () | (p) | (p, p, ...) | [p, ...]
    | record-pattern
    | constructor p | p infix-constructor p
    | name as p | name : t as p | p : t
```

Record patterns include `{x = p, y = q}`, puns like `{x, y}`, and `...` for
unlisted fields, as in `{x = p, ...}`. Type checking must resolve the full
record shape, or it is an error. `#x` selectors have the same requirement.

Patterns work in `val`, `fun`, `fn`, `case`, and `handle`. Constructors include
`::`, `SOME`, user datatypes, exceptions, and `ref`. Whether a name is a
constructor comes from scope, not capitalization. Real-literal patterns and
or-patterns are not accepted.

In source files:

- Duplicate names in a binding group or pattern are errors.
- Redundant rules are errors.
- Non-exhaustive `fun` and `case` matches are warnings.

The REPL doesn't run these checks. At run time a failed `fun` or `case` match
raises `Match`, and a failed `val` pattern raises `Bind`.

## Declarations and types

```text
declaration ::= val [rec] p = e [and ...]
              | fun name p ... [: t] = e [| name p ... = e ...] [and ...]
              | type type-parameters name = t [and ...]
              | datatype datatype-bindings [withtype type-bindings]
              | datatype name = datatype qualified-name
              | abstype datatype-bindings [withtype type-bindings]
                  with declarations end
              | exception name [of t] [and ...]
              | exception name = qualified-name [and ...]
              | local declarations in declarations end
              | infix [0..9] names | infixr [0..9] names | nonfix names

datatype-binding ::= type-parameters name = constructor [of t] [| ...]
type-parameters  ::= empty | 'a | ('a, 'b, ...)
t ::= 'a | ''a | type-name | t type-name | (t, t, ...) type-name
     | t * t | t -> t | {label : t, ...} | (t)
```

- `and` groups bind simultaneously and allow mutually recursive functions and
  datatypes.
- A non-recursive `val` right-hand side doesn't see the names it binds.
- `val rec` requires `fn` right-hand sides.
- Types are inferred with Hindley–Milner polymorphism, equality constraints,
  overloading, and the value restriction.
- Explicit type variables after `val` or `fun` (`val 'a id = ...`) are not
  parsed.
- `datatype` creates a fresh type. Replication shares the original, except that
  replicating built-in types like `list` is not implemented.
- `abstype` hides constructors outside its body and removes equality.

Each evaluation of an `exception` declaration creates a new constructor;
replication shares it. The built-in exceptions are `Div`, `Overflow`, `Match`,
`Bind`, `Fail` (carries a string), `Subscript`, and `Empty`. The compiler raises
the first four itself. Nothing raises `Subscript` or `Empty` yet.

## Modules

Structures, signatures, and functors work, including nested structures, aliases,
qualified names, `open`, and `and` groups. Signature and functor declarations
are allowed at top level or inside a top-level `local`, not inside a structure
or `let`.

```sml
signature COUNTER = sig
  type t
  val zero : t
  val next : t -> t
end

functor MakeCounter (X : sig val step : int end) :> COUNTER = struct
  type t = int
  val zero = 0
  fun next n = n + X.step
end

structure Counter = MakeCounter (struct val step = 1 end)
```

- Signatures support `val`, `type`, `eqtype`, `datatype`, datatype replication,
  `exception`, `structure`, `include`, `sharing type`, and structure sharing,
  plus `where type` refinement. `withtype` in signatures is not accepted.
- Transparent (`:`) and opaque (`:>`) ascription both work.
- Functors take named parameters (`F (X : SIG)`) or specification parameters
  (`F (val x : int)`), and structure or declaration arguments. Each application
  gets fresh datatypes and exceptions.
- Functors in a top-level `local` are an SML/NJ extension that MLton rejects.
- Higher-order functors are not implemented.
- The REPL keeps module bindings between inputs but doesn't echo module
  declarations the way SML/NJ does.

## The Basis subset

Most of the Basis is SML under `basis/`, compiled ahead of every program. It
covers `General`, `Bool`, `Int`, `Char`, `Real`, `String`, `TextIO`, `Word8`,
`Posix.Process`, `OS.Process`, `List`, and `Option`, plus the top-level names
they export, such as `hd`, `map`, `@`, `ignore`, `o`, and `valOf`. Each
structure has only the functions in its source file.

Arrays, vectors, general file I/O, `Real.toString`, substrings, and most other
structures are not implemented. The compiler itself provides `print`, `size`,
`not`, `~`, `^`, `ref`, `!`, `:=`, equality, and the built-in constructors and
exceptions.

## REPL behavior

- Functions and heap values survive between inputs.
- Datatypes, exceptions, structures, signatures, functors, aliases, `open`,
  `local`, and fixity declarations are accepted and echoed. Fixity persists
  across inputs.
- Top-level semicolons separate transactions, including several phrases on one
  line.
- Printing stops at ten nested levels and twenty list elements. Abstract values
  print as `-`.
- `int` is 31 bits.

Examples are in [the fixtures](../tests/fixtures) and
[REPL transcripts](../tests/repl). When you change a language feature, update
its entry here and its fixtures.
