# <span id="section:0"></span>The `Option` structure

---

#### Synopsis

<span id="OPTION:SIG:SPEC"></span>
<span id="Option:STR:SPEC"></span>

```sml
signature OPTION
structure Option :> OPTION
```

The `Option` structure defines the [`option`](option.md#SIG:OPTION.option:TY:SPEC) type, used for handling partial functions and optional values, and provides a collection of common combinators.

The type, the `Option` exception, and the functions `getOpt`, `valOf`, and `isSome` are available in the top-level environment.

---

#### Interface

<span id="SIG:OPTION.option:TY:SPEC"></span>
<span id="SIG:OPTION.NONE:TY:SPEC"></span>
<span id="SIG:OPTION.SOME:TY:SPEC"></span>
<span id="SIG:OPTION.Option:EXN:SPEC"></span>
<span id="SIG:OPTION.getOpt:VAL:SPEC"></span>
<span id="SIG:OPTION.isSome:VAL:SPEC"></span>
<span id="SIG:OPTION.valOf:VAL:SPEC"></span>
<span id="SIG:OPTION.filter:VAL:SPEC"></span>
<span id="SIG:OPTION.join:VAL:SPEC"></span>
<span id="SIG:OPTION.app:VAL:SPEC"></span>
<span id="SIG:OPTION.map:VAL:SPEC"></span>
<span id="SIG:OPTION.mapPartial:VAL:SPEC"></span>
<span id="SIG:OPTION.compose:VAL:SPEC"></span>
<span id="SIG:OPTION.composePartial:VAL:SPEC"></span>

```sml
datatype 'a option = NONE | SOME of 'a
exception Option
val getOpt : 'a option * 'a -> 'a
val isSome : 'a option -> bool
val valOf : 'a option -> 'a
val filter : ('a -> bool) -> 'a -> 'a option
val join : 'a option option -> 'a option
val app : ('a -> unit) -> 'a option -> unit
val map : ('a -> 'b) -> 'a option -> 'b option
val mapPartial : ('a -> 'b option) -> 'a option -> 'b option
val compose : ('a -> 'b) * ('c -> 'a option) -> 'c -> 'b option
val composePartial : ('a -> 'b option) * ('c -> 'a option) -> 'c -> 'b option
```

#### Description

<span id="SIG:OPTION.option:TY"></span>**`datatype`**` `_`'a`_` option = NONE | SOME `**`of`**` `_`'a`_  
The type [`option`](option.md#SIG:OPTION.option:TY:SPEC) provides a distinction between some value and no value, and is often used for representing the result of partially defined functions. It can be viewed as a typed version of the C convention of returning a `NULL` pointer to indicate no value.

<span id="SIG:OPTION.getOpt:VAL"></span>

### `getOpt`

```sml
val getOpt : 'a option * 'a -> 'a
```
`getOpt (``opt``, ``a``) `  
returns `v` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`; otherwise it returns `a`.

```repl
Option.getOpt (NONE, 0);; (* 0 *)
```
<span id="SIG:OPTION.isSome:VAL"></span>

### `isSome`

```sml
val isSome : 'a option -> bool
```
`isSome ``opt`` `  
returns `true` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`; otherwise it returns `false`.

```repl
Option.isSome (SOME 1);; (* true *)
```
<span id="SIG:OPTION.valOf:VAL"></span>

### `valOf`

```sml
val valOf : 'a option -> 'a
```
`valOf ``opt`` `  
returns `v` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`; otherwise it raises the [`Option`](option.md#SIG:OPTION.Option:EXN:SPEC) exception.

```repl
Option.valOf (SOME 1);; (* 1 *)
```
<span id="SIG:OPTION.filter:VAL"></span>

### `filter`

```sml
val filter : ('a -> bool) -> 'a -> 'a option
```
`filter ``f`` ``a`` `  
returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``a``)` if `f(a)` is `true` and [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) otherwise.

```repl
Option.filter (fn n => n > 0) 2;; (* SOME 2 *)
```
<span id="SIG:OPTION.join:VAL"></span>

### `join`

```sml
val join : 'a option option -> 'a option
```
The [`join`](option.md#SIG:OPTION.join:VAL:SPEC) function maps [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) and [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)` to `v`.

```repl
Option.join (SOME (SOME 2));; (* SOME 2 *)
```
<span id="SIG:OPTION.app:VAL"></span>

### `app`

```sml
val app : ('a -> unit) -> 'a option -> unit
```
`app ``f`` ``opt`` `  
applies the function `f` to the value `v` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`, and otherwise does nothing.

```repl
Option.app print (SOME "hello");; (* prints hello *)
```
<span id="SIG:OPTION.map:VAL"></span>

### `map`

```sml
val map : ('a -> 'b) -> 'a option -> 'b option
```
`map ``f`` ``opt`` `  
maps [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) and `SOME(v)` to [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``f`` v)`.

```repl
Option.map (fn n => n + 1) (SOME 41);; (* SOME 42 *)
```
<span id="SIG:OPTION.mapPartial:VAL"></span>

### `mapPartial`

```sml
val mapPartial : ('a -> 'b option) -> 'a option -> 'b option
```
`mapPartial ``f`` ``opt`` `  
maps [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) and `SOME(v)` to `f``(v)`. The expression `mapPartial ``f` is equivalent to `join o (map ``f``)`.

```repl
Option.mapPartial Int.fromString (SOME "42");; (* SOME 42 *)
```
<span id="SIG:OPTION.compose:VAL"></span>

### `compose`

```sml
val compose : ('a -> 'b) * ('c -> 'a option) -> 'c -> 'b option
```
`compose (``f``, ``g``) ``a`` `  
returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `g``(``a``)` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC); otherwise, if `g``(``a``)` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``f`` v)`. Thus, the `compose` function composes `f` with the partial function `g` to produce another partial function. The expression `compose (``f``, ``g``)` is equivalent to `(map ``f``) o ``g`.

```repl
Option.compose (fn n => n + 1, fn s => Int.fromString s) "41";; (* SOME 42 *)
```
<span id="SIG:OPTION.composePartial:VAL"></span>

### `composePartial`

```sml
val composePartial : ('a -> 'b option) * ('c -> 'a option) -> 'c -> 'b option
```
`composePartial (``f``, ``g``) ``a`` `  
returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `g``(``a``)` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC); otherwise, if `g``(``a``)` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`, it returns `f``(v)`. Thus, the `composePartial` function composes the two partial functions `f` and `g` to produce another partial function. The expression `composePartial (``f``, ``g``)` is equivalent to `(mapPartial ``f``) o ``g`.

```repl
Option.composePartial (fn n => if n > 0 then SOME (n + 1) else NONE, Int.fromString) "41";; (* SOME 42 *)
```
