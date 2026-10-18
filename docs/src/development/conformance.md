# Core conformance inventory

Use this inventory to find executable evidence for language behavior and the
Beads issues for remaining gaps. It records representative coverage, not a claim
that Nassau fully conforms to SML'97.

Start with the local [manual](../manual/introduction.md) for implementation
requirements, especially the
[top-level environment](../manual/top-level-environment.md),
[General](../manual/general.md), [Integer](../manual/integer.md),
[Char](../manual/char.md), and [String](../manual/string.md) pages. The
[language support page](grammar.md) describes accepted language forms. Basis
interface expansion remains under `nassau-617`; this inventory covers the
language and its primitive runtime boundary.

## Run the coverage

```sh
cargo test --release --test fixtures -- grammar
cargo test --release --test fixtures -- conformance
cargo nextest r --release
```

Paths below are relative to [tests/fixtures](../../../tests/fixtures).
`grammar/` probes syntax productions. `parser/` adds AST snapshots. `types/`
checks inferred types and runs accepted programs. Other accepted fixtures
compare native and interpreter output with Poly/ML. `tests/repl/` checks
persistent interactive state against Poly/ML.

An `error/` fixture must be rejected. A `*.unsupported/` directory preserves a
known discrepancy: positive fixtures must be accepted by Poly/ML but still fail
in Nassau; negative fixtures must be rejected by Poly/ML but still lack a Nassau
source diagnostic. The harness fails when a discrepancy disappears and prints
the move needed to promote the fixture. Regenerate expectations with
`python3 tools/update_filecheck.py 'tests/fixtures/<category>/**/*.sml'`.

## Find parser and static evidence

| Area                                                       | Evidence                                                                                                                                   | Remaining limitation                                                                               |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------- |
| Constants, escapes, nested comments, longest tokens        | `lexer/`, `lexical-strings.sml`, `lexical-numbers.sml`, `grammar/con/`                                                                     | Character set is 8-bit; Unicode escapes above 255 are rejected.                                    |
| Alphanumeric, symbolic, qualified and `op` identifiers     | `grammar/id/`, `grammar/exp/op-longid.sml`, `grammar/pat/op-longid.sml`                                                                    | Constructor status is preserved in value patterns.                                                 |
| Fixity, precedence, associativity and local scope          | `parser/valid-operators.sml`, `grammar/dec/`, `tests/repl/fixity.sml`                                                                      | Infix constructor expressions and exceptions are supported.                                        |
| Atomic, record, application, typed and control expressions | `grammar/exp/`, `parser/`, `types/valid-control.sml`                                                                                       | The empty record type is unit.                                                                     |
| Matches, constructor status, typed and layered patterns    | `grammar/pat/`, `patterns/`, `types/error/constructor-pattern-argument.sml`                                                                | Constructors cannot bind a layered pattern; record shape resolves at the generalizing declaration. |
| Arrow, tuple, record, alias and constructed types          | `grammar/typ/`, `types/error/type-arguments.sml`, `types/error/occurs-check.sml`                                                           | Duplicate labels and type parameters are rejected.                                                 |
| Simultaneous and recursive value bindings                  | `conformance/dynamic/binding-scope.sml`, `conformance/error/val-rec-non-fn.sml`                                                            | Implicit annotation scope: .25.                                                                    |
| Explicit type-variable sequences and equality variables    | `grammar/dec/*explicit-tyvar*.sml`, `types/explicit-scopes.sml`, `types/explicit-and.sml`, `types/error/explicit-*.sml`                    | Nested explicit shadowing follows Poly/ML.                                                         |
| Polymorphism, equality, overloading, value restriction     | `types/valid-polymorphism.sml`, `types/valid-value-restriction.sml`, `types/error/weak-type-conflict.sml`, `types/error/real-equality.sml` | Implicit annotation rigidity: .25.                                                                 |
| Types, datatypes, replication, `withtype`, `abstype`       | `datatypes/`, `types/valid-abstype.sml`, `types/error/abstype-hidden-constructor.sml`                                                      | Escaping local datatype: .26.                                                                      |
| Exception arguments, replication and handler types         | `types/error/exception-type-variable.sml`, `types/error/handler-result-type.sml`, `exceptions/`                                            | Protected value/constructor names are rejected in declarations and specifications.                 |
| Duplicate bindings, constructors and pattern variables     | `conformance/error/duplicate-*.sml`, `conformance/error/pattern-duplicate-variable.sml`                                                    | Includes labels, type parameters, specifications and module groups.                                |
| Structure aliases, `open`, local structure scope           | `modules/scopes-and-effects.sml`, `modules/valid-open.sml`                                                                                 | Duplicate names in a binding group are rejected.                                                   |
| Signatures, enrichment, transparent/opaque matching        | `modules/valid-signature-matching.sml`, `conformance/error/opaque-identity.sml`, `conformance/error/signature-value-status.sml`            | Duplicate specification names are rejected within each namespace.                                  |
| Include, refinement, type and structure sharing            | `grammar/spec/include-multiple.sml`, `modules/valid-where-type.sml`, `modules/valid-sharing.sml`                                           | Fresh signature names preserve sharing across nested substructures.                                |
| Functor arguments, matching and generative identities      | `functors/`, `types/functors/error/`, `conformance/error/functor-generativity.sml`                                                         | Duplicate names in a binding group are rejected.                                                   |
| Programs and persistent top-level state                    | `grammar/prog/`, `tests/repl/phrases.sml`, `tests/repl/modules.sml`                                                                        | File expressions require a final semicolon and bind `it`.                                          |

## Find dynamic evidence

| Behavior                                                                    | Evidence                                                                                      |
| --------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Function/argument, tuple, record and list evaluation order; short circuit   | `conformance/dynamic/evaluation-order.sml`                                                    |
| Simultaneous binding environments, closure capture, local export, recursion | `conformance/dynamic/binding-scope.sml`                                                       |
| Fresh exception identities, replication and effects before propagation      | `conformance/dynamic/exception-identity.sml`, `modules/exception-identities.sml`              |
| Constructor and reference patterns, `Match` and `Bind` failures             | `conformance/dynamic/pattern-failure.sml`, `patterns/`                                        |
| Structural equality, reference identity, compound equality                  | `equality/structural.sml`, `equality/polymorphic.sml`                                         |
| Mutable references, assignment and loops                                    | `references/`, `types/valid-imperative.sml`                                                   |
| Integer limits, overflow, signed division and zero division                 | `integers/`, `runtime/overflow.sml`, `runtime/div-by-zero.sml`                                |
| Fresh datatype/functor identities, abstraction and module initialization    | `functors/valid-generative.sml`, `modules/valid-opaque.sml`, `modules/scopes-and-effects.sml` |
| Recursion through closures and mutually recursive tail calls                | `functions/tail-calls.sml`, `functions/`, `tests/repl/tail-calls.sml`                         |
| Uncaught exception status and stored exceptions                             | `exceptions/uncaught*.sml`, `tests/repl/uncaught-div.sml`                                     |

## Resolve tracked gaps

Issue numbers below have the prefix `nassau-ugc`. Each unsupported fixture names
its issue in its header.

| Issue | Discrepancy                                                              | Fixture location                                                                                                      |
| ----- | ------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| .25   | Implicit annotation variables may specialize or receive the wrong scope. | `conformance.unsupported/error/implicit-tyvar-*.sml`, `conformance.unsupported/static/implicit-tyvar-inner-scope.sml` |
| .26   | A local datatype identity can escape a function result.                  | `conformance.unsupported/error/exception-type-escape.sml`                                                             |

The primitive Basis boundary also depends on `nassau-949.24` for standard
word-family support. `conformance.unsupported/static/word-arithmetic.sml`
records a core overloaded word expression that still fails. Arbitrary-precision
integer representation is tracked by `nassau-949.22`. The local Word and Integer
manual pages define those requirements; neither is implied by accepting small
integer and word literals.

## Account for implementation choices

- `int` has 31-bit signed precision. Boundary fixtures specify the required
  oracle precision; a different Poly/ML integer width cannot prove those cases.
  Follow the local Integer manual's precision-dependent limits.
- `char` is an 8-bit character, consistent with the local Char manual. String
  literals encode raw UTF-8 source as bytes; `\uXXXX` denotes one byte.
- Nassau follows Poly/ML in permitting a nested explicit binder to shadow an
  outer type-variable name. `types/explicit-scopes.sml` exercises this
  extension.
- Redundant source-file matches are errors; non-exhaustive function and case
  matches are warnings. The REPL omits these file-level checks. See
  `parser/exceptions/error/redundant-handler.sml` and `patterns/` for evidence.
  These diagnostic choices are separate from runtime `Match`/`Bind` behavior.
- Flexible-record resolution currently ends at the generalizing declaration.
  Poly/ML allows some later uses to fix the shape. This is the explicit policy
  decided in `nassau-ugc.21`. The rejection fixtures
  `grammar/error/flex-record-later-use.sml` and
  `grammar/error/flex-record-let.sml` skip the Poly/ML oracle with this reason;
  `grammar/pat/flex-record-declaration-scope.sml` exercises accepted constraints
  inside the declaration. The REPL applies the same declaration boundary.
- Higher-order functors, or-patterns and other language extensions are outside
  the SML'97 target. Optional Basis interfaces remain separately scoped by the
  local manual and Beads.
