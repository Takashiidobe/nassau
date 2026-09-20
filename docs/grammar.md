# Language support

Nassau can compile the core SML constructs used to write functions, build data,
match patterns, mutate references, and handle exceptions. Structures,
signatures, and functors compile to native code too.

This document describes what works today. The grammar sketches show the forms
the implementation accepts; they are not a complete grammar from the SML
Definition. See [architecture.md](architecture.md) for how these forms move
through the compiler.

## What works where

“Native” means compiling a source file into an executable. The REPL uses the
same lowering and code generation, with additional limits on how declarations
and values are displayed.

| Form                                                        | Parser and type checker      | Native                              | REPL                                                        |
| ----------------------------------------------------------- | ---------------------------- | ----------------------------------- | ----------------------------------------------------------- |
| Integers, reals, strings, characters, booleans, unit        | Yes                          | Yes                                 | Yes                                                         |
| Word literals and equality                                  | Yes                          | Limited to supported word constants | Same as native                                              |
| Word arithmetic and ordering                                | Yes                          | Unsupported                         | Unsupported                                                 |
| Tuples, records, lists, selectors                           | Yes                          | Yes                                 | Yes                                                         |
| Application, `fn`, `fun`, recursion, currying               | Yes                          | Yes                                 | Yes                                                         |
| `if`, `case`, `andalso`, `orelse`, sequences                | Yes                          | Yes                                 | Yes                                                         |
| `val`, `let`, `local`, type annotations and aliases         | Yes                          | Yes                                 | Yes                                                         |
| Datatypes, `withtype`, `abstype`, user datatype replication | Yes                          | Yes                                 | Yes              |
| Exceptions, replication, `raise`, `handle`                  | Yes                          | Yes                                 | Yes |
| References, assignment, `while`, `before`                   | Yes                          | Yes                                 | Yes                                                         |
| Fixity declarations and user infix functions                | Yes                          | Yes                                 | Yes; fixity persists between inputs             |
| Structures, signatures, `open`, module-qualified names      | Yes                          | Yes                                 | Yes              |
| Functors and structure/type sharing                         | Yes                          | Yes                                 | Yes              |
| Full Standard Basis Library                                 | A small subset is recognized | A smaller subset is executable      | Same as native                                              |

## Tokens and literals

Names can be alphanumeric (`count`, `value'`, `item_2`) or symbolic (`++`, `!`).
Type variables use a leading apostrophe, such as `'a`; equality type variables
use two, such as `''a`. Qualified names such as `S.value` and `S.t` are accepted
by the front end.

Comments use `(* ... *)` and may nest. Strings use double quotes and characters
use `#"..."`. The lexer handles the usual escapes (`\n`, `\t`, `\r`, `\"`,
`\\`, and the other control-character escapes), three-digit decimal escapes,
control escapes such as `\^A`, and whitespace gaps delimited by backslashes.

| Literal   | Examples                | Current behavior                                                    |
| --------- | ----------------------- | ------------------------------------------------------------------- |
| Integer   | `42`, `~42`, `0x2a`     | Signed 31-bit range: `~1073741824` through `1073741823`             |
| Real      | `1.5`, `1e3`, `~1.2e~3` | Stored as an IEEE 64-bit floating-point value                       |
| Word      | `0w42`, `0wx2a`         | Basic constants, matching, and equality; arithmetic is not compiled |
| String    | `"hello"`, `"a\000b"`   | Heap values; printing, concatenation, length, and equality work     |
| Character | `#"a"`, `#"\n"`         | Immediate values; matching and equality work                        |
| Boolean   | `true`, `false`         | Built-in constructors                                               |
| Unit      | `()`                    | Also the empty record `{}`                                          |

The lexer can recognize integers larger than the supported `int` range; the
parser rejects them as expressions or patterns. Arbitrary-precision integers
are not implemented. Word constant support is limited by the backend's
signed-word parsing, and the lexer's uppercase `0wX` spelling is not handled
by that parser.

Files normally contain declarations, with optional semicolons between them.
A file containing just one integer is also accepted as a legacy shortcut for a
program's exit status. The REPL accepts semicolon-terminated declarations and
expression phrases, which bind their result to `it`.

## Expressions

Here, `e` means an expression, `p` a pattern, and `t` a type. Repeated forms can
contain further expressions or declarations.

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

Functions can capture local values, be returned from functions, and be stored
in data structures. Curried functions support partial application. A saturated
call to a known curried function goes directly to its worker, avoiding
intermediate closures. Tail calls, including mutual recursion and calls through
closures, run without growing the stack.

The compiled numeric operations are `+`, `-`, `*`, `~`, `div`, and `mod` on
integers, and `+`, `-`, `*`, `~`, and `/` on reals. Integer `div` and `mod` round
according to SML's division rules. Arithmetic raises `Overflow` outside the
31-bit integer range and `Div` for integer division by zero. Numeric ordering
(`<`, `<=`, `>`, `>=`) is compiled for integers and reals.

`=` and `<>` support equality types: compound values compare structurally,
while references compare by identity. Functions and reals are not equality
types. `andalso` and `orelse` short-circuit. Sequences evaluate in order, and
`while` requires a boolean condition and a unit-valued body.

The parser starts with these infix declarations; higher numbers bind tighter:

| Precedence | Operators        | Associativity |
| ---------- | ---------------- | ------------- |
| 7          | `* / div mod`    | Left          |
| 6          | `+ - ^`          | Left          |
| 5          | `:: @`           | Right         |
| 4          | `= <> > >= < <=` | Left          |
| 3          | `:= o`           | Left          |
| 0          | `before`         | Left          |

Application binds tighter than infix operators. `andalso`, `orelse`, and
`handle` have their own parser levels. An operator's presence in the fixity
table does not guarantee a compiled implementation: list append `@`, for
example, is recognized but cannot be run unless supplied by user code.

## Patterns

```text
p ::= _ | name | op name | non-real literal
    | () | (p) | (p, p, ...) | [p, ...]
    | record-pattern
    | constructor p | p infix-constructor p
    | name as p | name : t as p | p : t
```

Record patterns include `{x = p, y = q}`, punned fields such as `{x, y}`,
typed and layered puns, and a trailing `...` for unspecified fields, as in
`{x = p, ...}`. The full record shape must be resolved by type checking; an
unresolved flexible record is an error. Selectors such as `#x` have the same
requirement.

Patterns work in `val`, `fun`, `fn`, `case`, and `handle`. Constructor patterns
include `::`, `SOME`, user datatypes, exceptions, and `ref`. Constructor status
comes from the environment, not from capitalization. User infix constructors
are supported. Real literals and SML/NJ-style or-patterns are not accepted as
patterns.

For source files, duplicate names within a binding group or pattern and
redundant match rules are errors. Non-exhaustive function and case matches
produce warnings. These checks live
outside the REPL's chunk pipeline, so its diagnostics do not yet have the same
coverage. At execution time, a failed function or case match raises `Match`, and
a failed `val` pattern raises `Bind`. An unmatched handler propagates the
original exception.

## Core declarations and types

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

`and` groups support simultaneous value and type bindings, and mutually
recursive functions and datatypes. A non-recursive `val` right-hand side sees the
previous environment, not the names it is about to bind. `val rec` requires
`fn` right-hand sides. Function clauses must agree on the function name and
number of parameters; prefix and infix definitions both work.

Types are inferred using Hindley–Milner polymorphism, equality constraints,
overloading, and the value restriction. Type application binds tighter than
products, which bind tighter than the right-associative arrow. Type aliases,
record types, parameterized datatypes, and annotations on expressions and
patterns are supported. Explicit type-variable sequences after `val` or `fun`
(such as `val 'a id = ...`) are not parsed yet.

`datatype` declarations create fresh type identities. Datatype replication
shares the original identity and constructors; replication of built-in
datatypes such as `list` is not implemented. `abstype` hides its constructors
outside the body and prevents equality on the abstract type.

Each evaluation of a fresh `exception` declaration creates a new constructor
identity. Exception replication shares that identity. Exceptions can carry
values, be stored in data, and be passed between functions. Built-in exception
constructors are `Div`, `Overflow`, `Match`, `Bind`, `Fail`, `Subscript`, and
`Empty`; `Fail` carries a string. The compiler automatically raises the first
four where appropriate. Array subscripting and the Basis operations that would
raise `Subscript` or `Empty` are not implemented yet.

## Modules

The front end accepts structures, signatures, and functors, including nested
structures, aliases, qualified names, `open`, `and` groups, and structure-local
`let`. Signature and functor declarations are allowed at top level or inside a
top-level `local`, not inside a structure or expression-level `let`.

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

Structures compile to their exported value bindings. Aliases reuse those
bindings, and `open` brings them into the current scope. Signatures restrict
exports without creating runtime objects. Each functor application compiles
a separate typed body in the environment of its declaration, with the
argument bound to its parameter. Datatypes and exceptions in the body have
fresh identities per application. The REPL keeps module bindings between
inputs, but does not yet echo module declarations like SML/NJ.

Signature specifications support `val`, `type`, `eqtype`, `datatype`, datatype
replication, `exception`, `structure`, `include`, `sharing type`, and structure
sharing. Signature refinement uses `where type`. Transparent (`:`) and opaque
(`:>`) ascription are implemented in the checker, including constructor
visibility, equality requirements, and fresh abstract type identities.
`withtype` in signature specifications is not accepted.

Functors support named parameters (`F (X : SIG)`), specification parameters
(`F (val x : int)`), structure arguments, declaration arguments, and transparent
or opaque result signatures. Bodies are checked against the parameter
signature; applications elaborate the body with the supplied argument and
fresh identities where required. Functors in top-level `local` declarations
are a supported SML/NJ extension; MLton rejects that form. Higher-order
functors are not implemented.

## The Basis subset

Most of the Basis is SML source under `basis/`, checked and compiled ahead of
every program (see [architecture.md](architecture.md)). It provides the
structures `General`, `Bool`, `Int`, `Char`, `Real`, `String`, `TextIO`, `Word8`,
`Posix.Process`, `OS.Process`, `List` and `Option`, with the top-level names the Basis exposes
from them, such as `hd`, `map`, `@`, `ignore`, `o` and `valOf`. A structure
holds only the functions listed in its source file.

A few names are known to the compiler: `print`, `size`, `not`, `~`, `^`, `ref`,
`!`, `:=`, equality, and the built-in datatype constructors (booleans,
`nil`/`::`, `NONE`/`SOME`, `LESS`/`EQUAL`/`GREATER`) and exceptions. The
prelude reaches the primitives behind its structures as `Prim.name`.

Arrays, vectors, general file I/O, `Real.toString`, substrings, and most other
Basis structures remain outside the implemented subset.

## REPL and compatibility limits

Functions and heap values survive between REPL inputs. Datatypes, exceptions,
structures, signatures, functors, aliases, replication, `open`, `local`, and
fixity declarations are accepted and echoed. Fixity persists across inputs;
private fixity stays local and public fixity is exported. Top-level semicolons
separate transactions, including multiple phrases on the same input line.

Values are decoded using their inferred types, including user constructors
and exception payloads whose declaring scope has ended. Printing is bounded
to ten recursive levels and twenty list elements; abstract values print as
`-`. Nassau keeps its compact value layout and runtime exception diagnostics.
Oracle fixtures compare value and declaration echoes with Poly/ML while
ignoring layout whitespace outside quoted strings.

Nassau uses 31-bit `int` values. Integer-limit fixtures compare with Poly/ML
only when its `Int.precision` matches, and always check Nassau with FileCheck.
Poly/ML validates the exception-values and opaque functor-result fixtures.
The where-type arity error retains FileCheck coverage with an explicit oracle
exclusion because Poly/ML 5.9.2 accepts it. Runtime diagnostics, truncation and abstract-value formatting use retained
Nassau FileCheck expectations.
There is no garbage collector yet; heap values remain
allocated for the lifetime of the process.

Examples and regression coverage live in [the fixtures](../tests/fixtures)
and [REPL transcripts](../tests/repl). When a language feature changes, update
its entry here alongside its fixtures. Beads remains the place to track the
implementation work.
