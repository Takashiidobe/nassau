# <span id="section:0"></span>The `IntInf` structure

---

#### Synopsis

<span id="INT_INF:SIG:SPEC"></span>
<span id="IntInf:STR:SPEC"></span>

```sml
signature INT_INF (* OPTIONAL *)
structure IntInf :> INT_INF (* OPTIONAL *)
```

The optional `IntInf` structure is one of the possible implementations of the [`INTEGER`](integer.md#INTEGER:SIG:SPEC) interface. In addition to the [`INTEGER`](integer.md#INTEGER:SIG:SPEC) operations, it provides some operations useful for programming with arbitrarily large integers. Operations in `IntInf` that return a value of type [`IntInf.int`](integer.md#SIG:INTEGER.int:TY:SPEC) should never raise the [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) exception. Note that, as it extends the [`INTEGER`](integer.md#INTEGER:SIG:SPEC) interface, `IntInf` defines a type `int`. Any use of this type below, unmodified by a structure, refers to the local type `int` defined in `IntInf`.

---

#### Interface

<span id="SIG:INT_INF.divMod:VAL:SPEC"></span>
<span id="SIG:INT_INF.quotRem:VAL:SPEC"></span>
<span id="SIG:INT_INF.pow:VAL:SPEC"></span>
<span id="SIG:INT_INF.log2:VAL:SPEC"></span>
<span id="SIG:INT_INF.orb:VAL:SPEC"></span>
<span id="SIG:INT_INF.xorb:VAL:SPEC"></span>
<span id="SIG:INT_INF.andb:VAL:SPEC"></span>
<span id="SIG:INT_INF.notb:VAL:SPEC"></span>
<span id="SIG:INT_INF.\|@LT\|\|@LT\|:VAL:SPEC"></span>
<span id="SIG:INT_INF.~\|@GT\|\|@GT\|:VAL:SPEC"></span>

```sml
include INTEGER
val divMod : int * int -> int * int
val quotRem : int * int -> int * int
val pow : int * Int.int -> int
val log2 : int -> Int.int
val orb : int * int -> int
val xorb : int * int -> int
val andb : int * int -> int
val notb : int -> int
val << : int * Word.word -> int
val ~>> : int * Word.word -> int
```

#### Description

<span id="SIG:INT_INF.divMod:VAL"></span>
`divMod (``i``, ``j``) `  
returns the pair `(``i`` `[`div`](integer.md#SIG:INTEGER.div:VAL:SPEC)` ``j``, ``i`` `[`mod`](integer.md#SIG:INTEGER.mod:VAL:SPEC)` ``j``)`, but is likely to be more efficient than computing both components separately. It raises [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) if `j` = 0.

<span id="SIG:INT_INF.quotRem:VAL"></span>
`quotRem (``i``, ``j``) `  
returns the pair `(``i`` `[`quot`](integer.md#SIG:INTEGER.quot:VAL:SPEC)` ``j``, ``i`` `[`rem`](integer.md#SIG:INTEGER.rem:VAL:SPEC)` ``j``)`, but is likely to be more efficient than computing both components separately. It raises [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) if `j` = 0.

<span id="SIG:INT_INF.pow:VAL"></span>
`pow (``i``, ``j``) `  
returns the result of raising `i` to the `j`<sup>(th)</sup> power. This is well-defined when `j` \> 0. When `j` = 0, [`pow`](int-inf.md#SIG:INT_INF.pow:VAL:SPEC)`(``i``, ``j``)` is 1; in particular, [`pow`](int-inf.md#SIG:INT_INF.pow:VAL:SPEC)`(0, 0)` is 1. When `j` \< 0, we define the following exceptional cases:

---

`i`

`pow(``i``,``j``)`

0

Raise [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC)

\|`i`\| = 1

`i`<sup>(`j`)</sup>

\|`i`\| \> 1

0

---


<span id="SIG:INT_INF.log2:VAL"></span>
`log2 ``i`` `  
returns the truncated base-2 logarithm of its argument, _i.e._, the largest integer `k` for which `pow`(2, `k`) \<= `i`. It raises [`Domain`](general.md#SIG:GENERAL.Domain:EXN:SPEC) if `i` \<= 0 and [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the result is not representable as an [`Int.int`](integer.md#SIG:INTEGER.int:TY:SPEC).

<span id="SIG:INT_INF.orb:VAL"></span>**`val`**` orb `**`:`**` int `**`*`**` int `**`->`**` int`
**`val`**` xorb `**`:`**` int `**`*`**` int `**`->`**` int`
**`val`**` andb `**`:`**` int `**`*`**` int `**`->`**` int`  
These functions return the bit-wise OR, bit-wise exclusive OR, and bit-wise AND, respectively, of the arguments.

<span id="SIG:INT_INF.notb:VAL"></span>
`notb ``i`` `  
returns the bit-wise complement (NOT) of `i`. It is equivalent to `~(``i`` + 1)`.

<span id="SIG:INT_INF.\|@LT\|\|@LT\|:VAL"></span>
`<< (``i``, ``n``) `  
shifts `i` to the left by `n` bit positions, filling in zeros from the right. When `i` and `n` are interpreted as integers, the latter non-negative, this returns (`i` \* 2<sup>(`n`)</sup>).

<span id="SIG:INT_INF.~\|@GT\|\|@GT\|:VAL"></span>
`~>> (``i``, ``n``) `  
shifts `i` to the right by `n` bit positions. When `i` and `n` are interpreted as integers, the latter non-negative, this returns **floor**(((`i` / 2<sup>(`n`)</sup>))).

#### Examples

```repl
IntInf.pow (2, 100);;
```

#### See Also

> [`INTEGER`](integer.md#INTEGER:SIG:SPEC), [`LargeInt`](integer.md#LargeInt:STR:SPEC)

#### Discussion

If an implementation provides the [`IntInf`](int-inf.md#IntInf:STR:SPEC) structure, then the type [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC) must be the same as the type [`IntInf.int`](integer.md#SIG:INTEGER.int:TY:SPEC).

The bit-wise operations (`andb`, `orb`, `notb`, `<<`, etc.) treat the integer arguments as having 2's complement representation. In particular, if we let bit = 2<sup>(n)</sup>, we have, for all sufficiently large values of n,

---

[`andb`](int-inf.md#SIG:INT_INF.andb:VAL:SPEC)(i, bit) = 0 if i \>= 0

[`andb`](int-inf.md#SIG:INT_INF.andb:VAL:SPEC)(i, bit) = bit if i \< 0

---

> **Rationale:**
>
> It is useful to have a module providing bit-wise operations on an unbounded domain. Such a module can serve as the basis for implementing sets or bit-vectors. These operations seemed to naturally fit into the specification of the `IntInf` module, rather than require an additional `WordInf` structure.

> **Implementation note:**
>
> Having this structure as part of the basis allows implementations to provide compiler or runtime support to optimize integer representation and operations.
