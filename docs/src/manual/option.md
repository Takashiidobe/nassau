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
`getOpt (``opt``, ``a``) `  
returns `v` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`; otherwise it returns `a`.

<span id="SIG:OPTION.isSome:VAL"></span>
`isSome ``opt`` `  
returns `true` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`; otherwise it returns `false`.

<span id="SIG:OPTION.valOf:VAL"></span>
`valOf ``opt`` `  
returns `v` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`; otherwise it raises the [`Option`](option.md#SIG:OPTION.Option:EXN:SPEC) exception.

<span id="SIG:OPTION.filter:VAL"></span>
`filter ``f`` ``a`` `  
returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``a``)` if `f(a)` is `true` and [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) otherwise.

<span id="SIG:OPTION.join:VAL"></span>**`val`**` join `**`:`**` `_`'a`_` option option `**`->`**` `_`'a`_` option`  
The [`join`](option.md#SIG:OPTION.join:VAL:SPEC) function maps [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) and [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)` to `v`.

<span id="SIG:OPTION.app:VAL"></span>
`app ``f`` ``opt`` `  
applies the function `f` to the value `v` if `opt` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`, and otherwise does nothing.

<span id="SIG:OPTION.map:VAL"></span>
`map ``f`` ``opt`` `  
maps [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) and `SOME(v)` to [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``f`` v)`.

<span id="SIG:OPTION.mapPartial:VAL"></span>
`mapPartial ``f`` ``opt`` `  
maps [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) to [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) and `SOME(v)` to `f``(v)`. The expression `mapPartial ``f` is equivalent to `join o (map ``f``)`.

<span id="SIG:OPTION.compose:VAL"></span>
`compose (``f``, ``g``) ``a`` `  
returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `g``(``a``)` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC); otherwise, if `g``(``a``)` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``f`` v)`. Thus, the `compose` function composes `f` with the partial function `g` to produce another partial function. The expression `compose (``f``, ``g``)` is equivalent to `(map ``f``) o ``g`.

<span id="SIG:OPTION.composePartial:VAL"></span>
`composePartial (``f``, ``g``) ``a`` `  
returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `g``(``a``)` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC); otherwise, if `g``(``a``)` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(v)`, it returns `f``(v)`. Thus, the `composePartial` function composes the two partial functions `f` and `g` to produce another partial function. The expression `composePartial (``f``, ``g``)` is equivalent to `(mapPartial ``f``) o ``g`.

#### Examples

```repl
Option.map (fn n => n + 1) (SOME 41);;
Option.getOpt (NONE, 0);;
```
