# <span id="section:0"></span>The `INTEGER` signature

---

#### Synopsis

<span id="INTEGER:SIG:SPEC"></span>
<span id="Int:STR:SPEC"></span>
<span id="FixedInt:STR:SPEC"></span>
<span id="LargeInt:STR:SPEC"></span>
<span id="Int{N}:STR:SPEC"></span>
<span id="Position:STR:SPEC"></span>

```sml
signature INTEGER
structure Int :> INTEGER
where type int = int
structure FixedInt :> INTEGER (* OPTIONAL *)
structure LargeInt :> INTEGER
structure Int<N> :> INTEGER (* OPTIONAL *)
structure Position :> INTEGER
```

Instances of the `INTEGER` signature provide a type of signed integers of either a fixed or arbitrary precision, and arithmetic and conversion operations. For fixed precision implementations, most arithmetic operations raise the exception [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when their result is not representable.

---

#### Interface

<span id="SIG:INTEGER.int:TY:SPEC"></span>
<span id="SIG:INTEGER.toLarge:VAL:SPEC"></span>
<span id="SIG:INTEGER.fromLarge:VAL:SPEC"></span>
<span id="SIG:INTEGER.toInt:VAL:SPEC"></span>
<span id="SIG:INTEGER.fromInt:VAL:SPEC"></span>
<span id="SIG:INTEGER.precision:VAL:SPEC"></span>
<span id="SIG:INTEGER.minInt:VAL:SPEC"></span>
<span id="SIG:INTEGER.maxInt:VAL:SPEC"></span>
<span id="SIG:INTEGER.+:VAL:SPEC"></span>
<span id="SIG:INTEGER.-:VAL:SPEC"></span>
<span id="SIG:INTEGER.*:VAL:SPEC"></span>
<span id="SIG:INTEGER.div:VAL:SPEC"></span>
<span id="SIG:INTEGER.mod:VAL:SPEC"></span>
<span id="SIG:INTEGER.quot:VAL:SPEC"></span>
<span id="SIG:INTEGER.rem:VAL:SPEC"></span>
<span id="SIG:INTEGER.compare:VAL:SPEC"></span>
<span id="SIG:INTEGER.\|@LT\|:VAL:SPEC"></span>
<span id="SIG:INTEGER.\|@LTE\|:VAL:SPEC"></span>
<span id="SIG:INTEGER.\|@GT\|:VAL:SPEC"></span>
<span id="SIG:INTEGER.\|@GTE\|:VAL:SPEC"></span>
<span id="SIG:INTEGER.~:VAL:SPEC"></span>
<span id="SIG:INTEGER.abs:VAL:SPEC"></span>
<span id="SIG:INTEGER.min:VAL:SPEC"></span>
<span id="SIG:INTEGER.max:VAL:SPEC"></span>
<span id="SIG:INTEGER.sign:VAL:SPEC"></span>
<span id="SIG:INTEGER.sameSign:VAL:SPEC"></span>
<span id="SIG:INTEGER.fmt:VAL:SPEC"></span>
<span id="SIG:INTEGER.toString:VAL:SPEC"></span>
<span id="SIG:INTEGER.scan:VAL:SPEC"></span>
<span id="SIG:INTEGER.fromString:VAL:SPEC"></span>

```sml
eqtype int
val toLarge : int -> LargeInt.int
val fromLarge : LargeInt.int -> int
val toInt : int -> Int.int
val fromInt : Int.int -> int
val precision : Int.int option
val minInt : int option
val maxInt : int option
val + : int * int -> int
val - : int * int -> int
val * : int * int -> int
val div : int * int -> int
val mod : int * int -> int
val quot : int * int -> int
val rem : int * int -> int
val compare : int * int -> order
val < : int * int -> bool
val <= : int * int -> bool
val > : int * int -> bool
val >= : int * int -> bool
val ~ : int -> int
val abs : int -> int
val min : int * int -> int
val max : int * int -> int
val sign : int -> Int.int
val sameSign : int * int -> bool
val fmt : StringCvt.radix -> int -> string
val toString : int -> string
val scan : StringCvt.radix -> (char, 'a) StringCvt.reader -> (int, 'a) StringCvt.reader
val fromString : string -> int option
```

#### Description

<span id="SIG:INTEGER.toLarge:VAL"></span>

### `toLarge`

```sml
val toLarge : int -> LargeInt.int
```

### `fromLarge`

```sml
val fromLarge : LargeInt.int -> int
```
These convert between integer values of types [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) and [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC). The latter raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the value does not fit.

`Int`_`<M>`_`.fromLarge o ``Int`_`<N>`_`.toLarge` converts an integer from type `Int`_`<N>`_`.int` to `Int`_`<M>`_`.int`.


```repl
Int.toLarge 42;; (* 42 *)
Int.fromLarge 42;; (* 42 *)
```

<span id="SIG:INTEGER.toInt:VAL"></span>

### `toInt`

```sml
val toInt : int -> Int.int
```

### `fromInt`

```sml
val fromInt : Int.int -> int
```
These convert between integer values of types [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) and the default integer type. They raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the value does not fit.


```repl
Int.toInt 42;; (* 42 *)
Int.fromInt 42;; (* 42 *)
```

<span id="SIG:INTEGER.precision:VAL"></span>

### `precision`

```sml
val precision : Int.int option
```
If [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``n``)`, this denotes the number `n` of significant bits in type [`int`](integer.md#SIG:INTEGER.int:TY:SPEC), including the sign bit. If it is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) has arbitrary precision. The precision need not necessarily be a power of two.


```repl
Int.precision;; (* implementation precision, or NONE for arbitrary precision *)
```

<span id="SIG:INTEGER.minInt:VAL"></span>

### `minInt`

```sml
val minInt : int option
```

### `maxInt`

```sml
val maxInt : int option
```
The minimal (most negative) and the maximal (most positive) integers, respectively, representable by [`int`](integer.md#SIG:INTEGER.int:TY:SPEC). If a value is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) can represent all negative (respectively, positive) integers, within the limits of the heap size.

If [`precision`](integer.md#SIG:INTEGER.precision:VAL:SPEC) is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``n``)`, then we have `minInt` = -2<sup>(`n`-1)</sup> and `maxInt` = 2<sup>(`n`-1)</sup> - 1.


```repl
Int.minInt;; (* SOME minimum value or NONE *)
Int.maxInt;; (* SOME maximum value or NONE *)
```

<span id="SIG:INTEGER.+:VAL"></span>

### `+`

```sml
val + : int * int -> int
```

### `-`

```sml
val - : int * int -> int
```

### `*`

```sml
val * : int * int -> int
```
These functions return the sum, difference, and product, respectively, of the arguments. They raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when the result is not representable.


```repl
Int.+ (20, 22);; (* 42 *)
Int.- (50, 8);; (* 42 *)
Int.* (6, 7);; (* 42 *)
```

<span id="SIG:INTEGER.div:VAL"></span>

### `div`

```sml
val div : int * int -> int
```
returns the greatest integer less than or equal to the quotient of `i` by `j`, _i.e._, **floor**(((`i` / `j`))). It raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when the result is not representable, or [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) when `j` = 0. Note that rounding is towards negative infinity, not zero.


```repl
Int.div (~7, 3);; (* ~3 *)
```

<span id="SIG:INTEGER.mod:VAL"></span>

### `mod`

```sml
val mod : int * int -> int
```
returns the remainder of the division of `i` by `j`. It raises [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) when `j` = 0. When defined, `(``i`` `[`mod`](integer.md#SIG:INTEGER.mod:VAL:SPEC)` ``j``)` has the same sign as `j`, and

(`i` [div](integer.md#SIG:INTEGER.div:VAL:SPEC) `j`) \* `j` + (`i` [mod](integer.md#SIG:INTEGER.mod:VAL:SPEC) `j`) = `i`



```repl
Int.mod (~7, 3);; (* 2 *)
```

<span id="SIG:INTEGER.quot:VAL"></span>

### `quot`

```sml
val quot : int * int -> int
```
returns the truncated quotient of the division of `i` by `j`, _i.e._, it computes (`i` / `j`) and then drops any fractional part of the quotient. It raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when the result is not representable, or [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) when `j` = 0. Note that unlike [`div`](integer.md#SIG:INTEGER.div:VAL:SPEC), [`quot`](integer.md#SIG:INTEGER.quot:VAL:SPEC) rounds towards zero. In addition, unlike [`div`](integer.md#SIG:INTEGER.div:VAL:SPEC) and [`mod`](integer.md#SIG:INTEGER.mod:VAL:SPEC), neither [`quot`](integer.md#SIG:INTEGER.quot:VAL:SPEC) nor [`rem`](integer.md#SIG:INTEGER.rem:VAL:SPEC) are infix by default; an appropriate infix declaration would be `infix 7 quot rem`.

> **Implementation note:**
>
> This is the semantics of most hardware divide instructions, so [`quot`](integer.md#SIG:INTEGER.quot:VAL:SPEC) may be faster than [`div`](integer.md#SIG:INTEGER.div:VAL:SPEC).



```repl
Int.quot (~7, 3);; (* ~2 *)
```

<span id="SIG:INTEGER.rem:VAL"></span>

### `rem`

```sml
val rem : int * int -> int
```
returns the remainder of the division of `i` by `j`. It raises [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) when `j`` = 0`. `(``i`` `[`rem`](integer.md#SIG:INTEGER.rem:VAL:SPEC)` ``j``)` has the same sign as `i`, and it holds that

(`i` [quot](integer.md#SIG:INTEGER.quot:VAL:SPEC) `j`) \* `j` + (`i` [rem](integer.md#SIG:INTEGER.rem:VAL:SPEC) `j`) = `i`

This is the semantics of most hardware divide instructions, so [`rem`](integer.md#SIG:INTEGER.rem:VAL:SPEC) may be faster than [`mod`](integer.md#SIG:INTEGER.mod:VAL:SPEC).


```repl
Int.rem (~7, 3);; (* ~1 *)
```

<span id="SIG:INTEGER.compare:VAL"></span>

### `compare`

```sml
val compare : int * int -> order
```
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) when `i` is less than, equal to, or greater than `j`, respectively.


```repl
Int.compare (1, 2);; (* LESS *)
```

<span id="SIG:INTEGER.\|@LT\|:VAL"></span>

### `<`

```sml
val < : int * int -> bool
```

### `<=`

```sml
val <= : int * int -> bool
```

### `>`

```sml
val > : int * int -> bool
```

### `>=`

```sml
val >= : int * int -> bool
```
These return `true` if the corresponding relation holds between the two integers.


```repl
Int.< (1, 2);; (* true *)
Int.<= (2, 2);; (* true *)
Int.> (3, 2);; (* true *)
Int.>= (2, 2);; (* true *)
```

<span id="SIG:INTEGER.~:VAL"></span>

### `~`

```sml
val ~ : int -> int
```
returns the negation of `i`, _i.e._, (0 - `i`). It raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when the result is not representable. This can happen, for example, when [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) is an n-bit 2's-complement integer type, and `~` is applied to -2 <sup>(n-1)</sup>.


```repl
Int.~ 3;; (* ~3 *)
```

<span id="SIG:INTEGER.abs:VAL"></span>

### `abs`

```sml
val abs : int -> int
```
returns the absolute value (magnitude) of `i`. It raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when the result is not representable.


```repl
Int.abs (~3);; (* 3 *)
```

<span id="SIG:INTEGER.min:VAL"></span>

### `min`

```sml
val min : int * int -> int
```

### `max`

```sml
val max : int * int -> int
```
These return the smaller (respectively, larger) of the arguments.


```repl
Int.min (2, 3);; (* 2 *)
Int.max (2, 3);; (* 3 *)
```

<span id="SIG:INTEGER.sign:VAL"></span>

### `sign`

```sml
val sign : int -> Int.int
```
returns `~1`, `0`, or `1` when `i` is less than, equal to, or greater than `0`, respectively.


```repl
Int.sign (~3);; (* ~1 *)
Int.sign 0;; (* 0 *)
Int.sign 3;; (* 1 *)
```

<span id="SIG:INTEGER.sameSign:VAL"></span>

### `sameSign`

```sml
val sameSign : int * int -> bool
```
returns `true` if `i` and `j` have the same sign. It is equivalent to `(`[`sign`](integer.md#SIG:INTEGER.sign:VAL:SPEC)` ``i`` = `[`sign`](integer.md#SIG:INTEGER.sign:VAL:SPEC)` ``j``)`.


```repl
Int.sameSign (~2, ~3);; (* true *)
Int.sameSign (~2, 3);; (* false *)
```

<span id="SIG:INTEGER.fmt:VAL"></span>

### `fmt`

```sml
val fmt : StringCvt.radix -> int -> string
```

### `toString`

```sml
val toString : int -> string
```
These return a string containing a representation of `i` with `#"~"` used as the sign for negative numbers. The former formats the string according to `radix`, The hexadecimal digits 10 through 15 are represented as `#"A"` through `#"F"`, respectively. No prefix `"0x"` is generated for the hexadecimal representation. The second form is equivalent to [`fmt`](integer.md#SIG:INTEGER.fmt:VAL:SPEC)` `[`StringCvt.DEC`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)` ``i`.


```repl
Int.fmt StringCvt.HEX 255;; (* "FF" *)
Int.toString (~42);; (* "~42" *)
```

<span id="SIG:INTEGER.scan:VAL"></span>

### `scan`

```sml
val scan : StringCvt.radix -> (char, 'a) StringCvt.reader -> (int, 'a) StringCvt.reader
```

### `fromString`

```sml
val fromString : string -> int option
```
The first expression returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``i``,``rest``)` if an integer in the format denoted by `radix` can be parsed from a prefix of the character stream `strm` after skipping initial whitespace, where `i` is the value of the integer parsed and `rest` is the rest of the character stream. [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned otherwise. This function raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when an integer can be parsed, but is too large to be represented by type [`int`](integer.md#SIG:INTEGER.int:TY:SPEC).

The format that `scan` accepts depends on the `radix` argument. Regular expressions defining these formats are as follows:

---

**Radix**

**Format**

[`StringCvt.BIN`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

\[`+~-`\]<sup>?</sup>\[`0`-`1`\]<sup>+</sup>

[`StringCvt.OCT`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

\[`+~-`\]<sup>?</sup>\[`0`-`7`\]<sup>+</sup>

[`StringCvt.DEC`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

\[`+~-`\]<sup>?</sup>\[`0`-`9`\]<sup>+</sup>

[`StringCvt.HEX`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

\[`+~-`\]<sup>?</sup>(`0x` \| `0X`)<sup>?</sup>\[`0`-`9``a`-`f``A`-`F`\]<sup>+</sup>

---

Note that strings such as `"0xg"` and `"0x 123"` are scanned as [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(0)`, even using a hexadecimal radix.

The second expression returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``i``)` if an integer `i` in the format \[`+~-`\]<sup>?</sup>\[`0`-`9`\]<sup>+</sup> can be parsed from a prefix of the string `s`, ignoring initial whitespace; [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned otherwise. The function [`fromString`](integer.md#SIG:INTEGER.fromString:VAL:SPEC) raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when an integer can be parsed, but is too large to fit in type [`int`](integer.md#SIG:INTEGER.int:TY:SPEC). It is equivalent to the expression [`StringCvt.scanString`](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC)`(scan`[`StringCvt.DEC`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)`)`.

```repl
Int.scan StringCvt.DEC Substring.getc (Substring.full "~42");; (* SOME (~42, empty substring) *)
Int.fromString "42";; (* SOME 42 *)
Int.fromString "bad";; (* NONE *)
```


#### See Also

> [`IntInf`](int-inf.md#IntInf:STR:SPEC), [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC)

#### Discussion

Fixed precision representations are required to be 2's complement. Implementations of arbitrary precision should appear as 2's complement under conversion to and from words.

If an implementation provides the [`IntInf`](int-inf.md#IntInf:STR:SPEC) structure, then [`LargeInt`](integer.md#LargeInt:STR:SPEC) must be the same structure as [`IntInf`](int-inf.md#IntInf:STR:SPEC) (viewed through a thinning [`INTEGER`](integer.md#INTEGER:SIG:SPEC) signature). Otherwise, if [`LargeInt`](integer.md#LargeInt:STR:SPEC) is not the same as [`Int`](integer.md#Int:STR:SPEC), then there must be a structure [`Int`_`<N>`_](integer.md#Int%7BN%7D:STR:SPEC) equal to [`LargeInt`](integer.md#LargeInt:STR:SPEC).

The type [`FixedInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC) is the largest fixed precision integer supported, while the type [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC) is the largest integer supported. A structure [`Int`_`<N>`_](integer.md#Int%7BN%7D:STR:SPEC) implements `N`-bit integers. The type [`Position.int`](integer.md#SIG:INTEGER.int:TY:SPEC) is used to represent positions in files and I/O streams.

> **Implementation note:**
>
> It is recommended that compilers recognize the idiom of converting between integers of differing precisions using an intermediate representation (_e.g._, `Int31.fromLarge o Int8.toLarge`) and optimize these compositions.
