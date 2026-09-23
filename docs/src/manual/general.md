# The `General` structure

## Synopsis

```sml
signature GENERAL
structure General :> GENERAL
```

The `General` structure defines exceptions, datatypes, and functions used throughout the Standard ML Basis Library. Its types and values are also available unqualified at the top level.

## Interface

```sml
eqtype unit
type exn = exn
exception Bind
exception Match
exception Chr
exception Div
exception Domain
exception Fail of string
exception Overflow
exception Size
exception Span
exception Subscript
val exnName : exn -> string
val exnMessage : exn -> string
datatype order = LESS | EQUAL | GREATER
val ! : 'a ref -> 'a
val := : 'a ref * 'a -> unit
val o : ('b -> 'c) * ('a -> 'b) -> 'a -> 'c
val before : 'a * unit -> 'a
val ignore : 'a -> unit
```

## Description

### `unit`

The type `unit` has one value, written `()`. It is commonly used as the argument to an operation that takes no meaningful input, or as the result of an operation whose purpose is its side effect.

```repl
();;
```

### `exn`

The type `exn` is the type of values raised and handled as exceptions. It behaves like a datatype whose set of constructors can be extended by exception declarations.

```repl
exception Sample of int;;
```

### `Bind` and `Match`

`Bind` is raised when a pattern match in a `val` binding fails. `Match` is raised when a pattern match in a `case` expression or function application fails.

```repl
(case 1 of 0 => "zero") handle Match => "no match";;
```

### `Chr`

`Chr` indicates an attempt to create a character with a code outside the range supported by the character type.

```repl
chr 256 handle Chr => #"?";;
```

### `Div`

`Div` indicates an attempt to divide by zero. It replaces the `Mod` exception from the SML'90 Definition.

```repl
10 div 0 handle Div => 0;;
```

### `Domain`

`Domain` indicates that a mathematical function was called with an argument outside its domain. It replaces the `Sqrt` and `Ln` exceptions from the SML'90 Definition.

```repl
raise Domain handle Domain => ();;
```

### `Fail`

`Fail` carries a string describing an operation failure. It is provided for user code and libraries; Basis Library functions do not raise it.

```repl
raise Fail "example failure" handle Fail message => message;;
```

### `Overflow`

`Overflow` indicates that an arithmetic result cannot be represented. It replaces the `Abs`, `Exp`, `Neg`, `Prod`, `Quot`, and `Sum` exceptions from the SML'90 Definition.

```repl
raise Overflow handle Overflow => 0;;
```

### `Size`

`Size` indicates an attempt to create an aggregate data structure, such as an array, string, or vector, with a size that is too large or negative.

```repl
raise Size handle Size => ();;
```

### `Span`

`Span` indicates an attempt to apply `Substring.span` to incompatible substrings.

```repl
raise Span handle Span => ();;
```

### `Subscript`

`Subscript` indicates an out-of-range index, typically when accessing an element of a list, string, array, or vector.

```repl
List.nth ([10, 20], 3) handle Subscript => 0;;
```

### `exnName`

`exnName ex` returns a name for the exception `ex`. If exception constructors alias one another, the returned name may be that of any alias.

```sml
let
  exception E1
  exception E2 = E1
in
  exnName E2
end
```

The result may be either `"E1"` or `"E2"`.

### `exnMessage`

`exnMessage ex` returns a message for exception `ex`. Its precise format may vary between implementations and locales, but it includes the string returned by `exnName ex`.

For example, `exnMessage Div` is `"Div"`.

### `order`

Values of type `order` represent the result of comparing values in a linear ordering: `LESS`, `EQUAL`, or `GREATER`.

```repl
Int.compare (3, 7);;
```

### `!` and `:=`

`! r` returns the value stored in reference `r`. The expression `r := x` updates `r` to contain `x` and returns `()`.

```repl
val r = ref 1;;
r := 2;;
!r;;
```

### `o`

`f o g` is function composition: `(f o g) x` is equivalent to `f (g x)`.

```repl
(Int.toString o (fn n => n + 1)) 4;;
```

### `before`

`(x before y)` evaluates `x`, then `y`, and returns the value of `x`. This provides a compact way to sequence an expression whose result is not needed before returning another value.

```repl
(42 before print "side effect\n");;
```

### `ignore`

`ignore x` evaluates `x` and returns `()`, discarding the result. It is useful when a function such as `List.app` expects a callback returning `unit`, but the callback naturally returns another type.

```repl
ignore (1 + 2);;
```

## Discussion

Implementations may provide a compatibility mode that makes the SML'90 exceptions replaced by exceptions in this structure available at top level as aliases.

In Nassau, `unit`, exceptions, `order`, and reference operators are part of the language's predeclared environment. `General` provides the library functions `o`, `before`, and `ignore`; these are also bound at top level.

## See also

[`List`](./list.md)

Source: [The Standard ML Basis Library, `General` structure](https://smlfamily.github.io/Basis/general.html). Generated April 12, 2004; last modified February 20, 1997.
