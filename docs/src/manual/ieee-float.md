# <span id="section:0"></span>The `IEEEReal` structure

---

#### Synopsis

<span id="IEEE_REAL:SIG:SPEC"></span>
<span id="IEEEReal:STR:SPEC"></span>

```sml
signature IEEE_REAL
structure IEEEReal :> IEEE_REAL
```

The `IEEEReal` structure defines types associated with an IEEE implementation of floating-point numbers. In addition, it provides control for the floating-point hardware's rounding mode. Refer to the IEEE standard 754-1985 **\[CITE\]** and the ANSI/IEEE standard 854-1987 **\[CITE\]** for additional information.

---

#### Interface

<span id="SIG:IEEE_REAL.Unordered:EXN:SPEC"></span>
<span id="SIG:IEEE_REAL.real_order:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.LESS:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.EQUAL:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.GREATER:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.UNORDERED:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.float_class:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.NAN:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.INF:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.ZERO:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.NORMAL:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.SUBNORMAL:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.rounding_mode:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.TO_NEAREST:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.TO_NEGINF:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.TO_POSINF:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.TO_ZERO:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.setRoundingMode:VAL:SPEC"></span>
<span id="SIG:IEEE_REAL.getRoundingMode:VAL:SPEC"></span>
<span id="SIG:IEEE_REAL.decimal_approx:TY:SPEC"></span>
<span id="SIG:IEEE_REAL.toString:VAL:SPEC"></span>
<span id="SIG:IEEE_REAL.scan:VAL:SPEC"></span>
<span id="SIG:IEEE_REAL.fromString:VAL:SPEC"></span>

```sml
exception Unordered
datatype real_order = LESS | EQUAL | GREATER | UNORDERED
datatype float_class
= NAN
| INF
| ZERO
| NORMAL
| SUBNORMAL
datatype rounding_mode
= TO_NEAREST
| TO_NEGINF
| TO_POSINF
| TO_ZERO
val setRoundingMode : rounding_mode -> unit
val getRoundingMode : unit -> rounding_mode
type decimal_approx = {
class : float_class,
sign : bool,
digits : int list,
exp : int
}
val toString : decimal_approx -> string
val scan : (char, 'a) StringCvt.reader -> (decimal_approx, 'a) StringCvt.reader
val fromString : string -> decimal_approx option
```

#### Description

<span id="SIG:IEEE_REAL.setRoundingMode:VAL"></span>

### `setRoundingMode`

```sml
val setRoundingMode : rounding_mode -> unit
```

### `getRoundingMode`

```sml
val getRoundingMode : unit -> rounding_mode
```
These set and get the rounding mode of the underlying hardware. The IEEE standard requires [`TO_NEAREST`](ieee-float.md#SIG:IEEE_REAL.rounding_mode:TY:SPEC) as the default rounding mode.

> **Implementation note:**
>
> Some platforms do not support all of the rounding modes. An SML implementation built on these platforms will necessarily be non-conforming with, presumably, [`setRoundingMode`](ieee-float.md#SIG:IEEE_REAL.setRoundingMode:VAL:SPEC) raising an exception for the unsupported modes.



```repl
IEEEReal.getRoundingMode ();; (* TO_NEAREST *)
IEEEReal.setRoundingMode IEEEReal.TO_NEAREST;; (* () *)
```

<span id="SIG:IEEE_REAL.decimal_approx:TY"></span>**`type`**` decimal_approx = {`
`                        class `**`:`**` float_class,`
`                        sign `**`:`**` bool,`
`                        digits `**`:`**` int list,`
`                        exp `**`:`**` int`
`                      }`  
This type provides a structured decimal representation of a real. The `class` field indicates the real class. If `sign` is `true`, the number is negative. The integers in the `digits` list must be digits, _i.e._, between 0 and 9.

When `class` is `NORMAL` or `SUBNORMAL`, a value of type `decimal_approx` with `digits` = \[d<sub>(1)</sub>, d<sub>(2)</sub>, ..., d<sub>(n)</sub>\] corresponds to the real number s \* 0.d<sub>(1)</sub>d<sub>(2)</sub>...d<sub>(n)</sub> 10<sup>(exp)</sup>, where s is -1 if `sign` is `true` and 1 otherwise. When `class` is `ZERO` or `INF`, the value corresponds to zero or infinity, respectively, with its sign determined by `sign`. When `class` is `NAN`, the value corresponds to an unspecified NaN value.

<span id="SIG:IEEE_REAL.toString:VAL"></span>

### `toString`

```sml
val toString : decimal_approx -> string
```
returns a string representation of `d`. Assuming `digits` = \[d<sub>(1)</sub>, d<sub>(2)</sub>, ..., d<sub>(n)</sub>\] and ignoring the `sign` and `exp` fields, `toString` generates the following strings depending on the `class` field:

---

[`ZERO`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC)

`"0.0"`

[`NORMAL`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC)

"0.d<sub>(1)</sub>d<sub>(2)</sub>...d<sub>(n)</sub>"

[`SUBNORMAL`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC)

"0.d<sub>(1)</sub>d<sub>(2)</sub>...d<sub>(n)</sub>"

[`INF`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC)

`"inf"`

[`NAN`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC)

`"nan"`

---

If the `sign` field is `true`, a `#"~"` is prepended. If the `exp` field is non-zero and the `class` is [`NORMAL`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC) or [`SUBNORMAL`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC), the string `"E"^(Integer.toString exp)` is appended.

The composition [`toString`](ieee-float.md#SIG:IEEE_REAL.toString:VAL:SPEC)`o`[`REAL.toDecimal`](real.md#SIG:REAL.toDecimal:VAL:SPEC) is equivalent to [`REAL.fmt`](real.md#SIG:REAL.fmt:VAL:SPEC)` `[`StringCvt.EXACT`](string-cvt.md#SIG:STRING_CVT.realfmt:TY:SPEC).


```repl
IEEEReal.toString {class = IEEEReal.NORMAL, sign = false, digits = [1, 2, 5], exp = 0};; (* "0.125" *)
```

<span id="SIG:IEEE_REAL.scan:VAL"></span>

### `scan`

```sml
val scan : (char, 'a) StringCvt.reader -> (decimal_approx, 'a) StringCvt.reader
```

### `fromString`

```sml
val fromString : string -> decimal_approx option
```
These functions scan a decimal approximation from a prefix of a character source. Initial whitespace is ignored. The first reads from the character stream `src` using the character input function `getc`. It returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``d``, ``rest``)` if the decimal approximation `d` can be parsed; `rest` is the remainder of the character stream. [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned otherwise.

The second form uses the string `s` as input. It returns the decimal approximation on success and [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) otherwise. The [`fromString`](ieee-float.md#SIG:IEEE_REAL.fromString:VAL:SPEC) function is equivalent to `StringCvt.scanString scan`.

The functions accept real numbers with the following format:

> \[`+~-`\]<sup>?</sup>(\[`0`-`9`\]<sup>+</sup>`.`\[`0`-`9`\]<sup>+?</sup> \| `.`\[`0`-`9`\]<sup>+</sup>)(`e` \| `E`)\[`+~-`\]<sup>?</sup>\[`0`-`9`\]<sup>+?</sup>

The optional sign determines the value of the `sign` field, with a default of `false`. Initial zeros are stripped from the integer part and trailing zeros are stripped from the fractional part, yielding two lists `il` and `fl`, respectively, of digits. If `il` is non-empty, then `class` is set to [`NORMAL`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC), `digits` is set to `il``@``fl` with any trailing zeros removed and `exp` is set to the length of `il` plus the value of the scanned exponent, if any. If `il` is empty and so is `fl`, then `class` is set to [`ZERO`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC), `digits = []` and `exp = 0`. Finally, if `il` is empty but `fl` is not, let m be the number of leading zeros in `fl` and let `fl'` be `fl` after the leading zeros are removed. Then, `class` is set to [`NORMAL`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC), `digits` is set to `fl'` and `exp` is set to -m plus the value of the scanned exponent, if any.

They also accept the following string representations of non-finite values:

> \[`+~-`\]<sup>?</sup>(`inf` \| `infinity` \| `nan`)

where the alphabetic characters are case-insensitive. The optional sign determines the value of the `sign` field, with a default of `false`. In the first and second cases, `d` will have `class` set to [`INF`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC). In the third case, `class` is set to [`NAN`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC). In all these cases, `d` will have `digits = []` and `exp = 0`.

```repl
StringCvt.scanString IEEEReal.scan "1.25";; (* SOME decimal approximation *)
IEEEReal.fromString "1.25";; (* SOME decimal approximation *)
IEEEReal.fromString "not a number";; (* NONE *)
```


#### See Also

> [`REAL`](real.md#REAL:SIG:SPEC), [`MATH`](math.md#MATH:SIG:SPEC)

#### Discussion

Values of type [`decimal_approx`](ieee-float.md#SIG:IEEE_REAL.decimal_approx:TY:SPEC) are independent of any floating-point representation.
