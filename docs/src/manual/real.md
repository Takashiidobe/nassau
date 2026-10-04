# <span id="section:0"></span>The `REAL` signature

---

#### Synopsis

<span id="REAL:SIG:SPEC"></span>
<span id="Real:STR:SPEC"></span>
<span id="LargeReal:STR:SPEC"></span>
<span id="Real{N}:STR:SPEC"></span>

```sml
signature REAL
structure Real :> REAL
where type real = real
structure LargeReal :> REAL
structure Real<N> :> REAL (* OPTIONAL *)
```

The `REAL` signature specifies structures that implement floating-point numbers. The semantics of floating-point numbers should follow the IEEE standard 754-1985 **\[CITE\]** and the ANSI/IEEE standard 854-1987**\[CITE\]**. In addition, implementations of the `REAL` signature are required to use non-trapping semantics. Additional aspects of the design of the `REAL` and [`MATH`](math.md#MATH:SIG:SPEC) signatures were guided by the Floating-Point C Extensions**\[CITE\]** developed by the X3J11 ANSI committee and the lecture notes **\[CITE\]** by W. Kahan on the IEEE standard 754.

Although there can be many representations for NaN values, the Library models them as a single value and currently provides no explicit way to distinguish among them, ignoring the sign bit. Thus, in the descriptions below and in the [`Math`](math.md#Math:STR:SPEC) structure, we just refer to the NaN value.

---

#### Interface

<span id="SIG:REAL.real:TY:SPEC"></span>
<span id="SIG:REAL.radix:VAL:SPEC"></span>
<span id="SIG:REAL.precision:VAL:SPEC"></span>
<span id="SIG:REAL.maxFinite:VAL:SPEC"></span>
<span id="SIG:REAL.minPos:VAL:SPEC"></span>
<span id="SIG:REAL.minNormalPos:VAL:SPEC"></span>
<span id="SIG:REAL.posInf:VAL:SPEC"></span>
<span id="SIG:REAL.negInf:VAL:SPEC"></span>
<span id="SIG:REAL.+:VAL:SPEC"></span>
<span id="SIG:REAL.-:VAL:SPEC"></span>
<span id="SIG:REAL.*:VAL:SPEC"></span>
<span id="SIG:REAL./:VAL:SPEC"></span>
<span id="SIG:REAL.rem:VAL:SPEC"></span>
<span id="SIG:REAL.*+:VAL:SPEC"></span>
<span id="SIG:REAL.*-:VAL:SPEC"></span>
<span id="SIG:REAL.~:VAL:SPEC"></span>
<span id="SIG:REAL.abs:VAL:SPEC"></span>
<span id="SIG:REAL.min:VAL:SPEC"></span>
<span id="SIG:REAL.max:VAL:SPEC"></span>
<span id="SIG:REAL.sign:VAL:SPEC"></span>
<span id="SIG:REAL.signBit:VAL:SPEC"></span>
<span id="SIG:REAL.sameSign:VAL:SPEC"></span>
<span id="SIG:REAL.copySign:VAL:SPEC"></span>
<span id="SIG:REAL.compare:VAL:SPEC"></span>
<span id="SIG:REAL.compareReal:VAL:SPEC"></span>
<span id="SIG:REAL.\|@LT\|:VAL:SPEC"></span>
<span id="SIG:REAL.\|@LTE\|:VAL:SPEC"></span>
<span id="SIG:REAL.\|@GT\|:VAL:SPEC"></span>
<span id="SIG:REAL.\|@GTE\|:VAL:SPEC"></span>
<span id="SIG:REAL.==:VAL:SPEC"></span>
<span id="SIG:REAL.!=:VAL:SPEC"></span>
<span id="SIG:REAL.?=:VAL:SPEC"></span>
<span id="SIG:REAL.unordered:VAL:SPEC"></span>
<span id="SIG:REAL.isFinite:VAL:SPEC"></span>
<span id="SIG:REAL.isNan:VAL:SPEC"></span>
<span id="SIG:REAL.isNormal:VAL:SPEC"></span>
<span id="SIG:REAL.class:VAL:SPEC"></span>
<span id="SIG:REAL.toManExp:VAL:SPEC"></span>
<span id="SIG:REAL.fromManExp:VAL:SPEC"></span>
<span id="SIG:REAL.split:VAL:SPEC"></span>
<span id="SIG:REAL.realMod:VAL:SPEC"></span>
<span id="SIG:REAL.nextAfter:VAL:SPEC"></span>
<span id="SIG:REAL.checkFloat:VAL:SPEC"></span>
<span id="SIG:REAL.realFloor:VAL:SPEC"></span>
<span id="SIG:REAL.realCeil:VAL:SPEC"></span>
<span id="SIG:REAL.realTrunc:VAL:SPEC"></span>
<span id="SIG:REAL.realRound:VAL:SPEC"></span>
<span id="SIG:REAL.floor:VAL:SPEC"></span>
<span id="SIG:REAL.ceil:VAL:SPEC"></span>
<span id="SIG:REAL.trunc:VAL:SPEC"></span>
<span id="SIG:REAL.round:VAL:SPEC"></span>
<span id="SIG:REAL.toInt:VAL:SPEC"></span>
<span id="SIG:REAL.toLargeInt:VAL:SPEC"></span>
<span id="SIG:REAL.fromInt:VAL:SPEC"></span>
<span id="SIG:REAL.fromLargeInt:VAL:SPEC"></span>
<span id="SIG:REAL.toLarge:VAL:SPEC"></span>
<span id="SIG:REAL.fromLarge:VAL:SPEC"></span>
<span id="SIG:REAL.fmt:VAL:SPEC"></span>
<span id="SIG:REAL.toString:VAL:SPEC"></span>
<span id="SIG:REAL.scan:VAL:SPEC"></span>
<span id="SIG:REAL.fromString:VAL:SPEC"></span>
<span id="SIG:REAL.toDecimal:VAL:SPEC"></span>
<span id="SIG:REAL.fromDecimal:VAL:SPEC"></span>

```sml
type real
structure Math : MATH
where type real = real
val radix : int
val precision : int
val maxFinite : real
val minPos : real
val minNormalPos : real
val posInf : real
val negInf : real
val + : real * real -> real
val - : real * real -> real
val * : real * real -> real
val / : real * real -> real
val rem : real * real -> real
val *+ : real * real * real -> real
val *- : real * real * real -> real
val ~ : real -> real
val abs : real -> real
val min : real * real -> real
val max : real * real -> real
val sign : real -> int
val signBit : real -> bool
val sameSign : real * real -> bool
val copySign : real * real -> real
val compare : real * real -> order
val compareReal : real * real -> IEEEReal.real_order
val < : real * real -> bool
val <= : real * real -> bool
val > : real * real -> bool
val >= : real * real -> bool
val == : real * real -> bool
val != : real * real -> bool
val ?= : real * real -> bool
val unordered : real * real -> bool
val isFinite : real -> bool
val isNan : real -> bool
val isNormal : real -> bool
val class : real -> IEEEReal.float_class
val toManExp : real -> {man : real, exp : int}
val fromManExp : {man : real, exp : int} -> real
val split : real -> {whole : real, frac : real}
val realMod : real -> real
val nextAfter : real * real -> real
val checkFloat : real -> real
val realFloor : real -> real
val realCeil : real -> real
val realTrunc : real -> real
val realRound : real -> real
val floor : real -> int
val ceil : real -> int
val trunc : real -> int
val round : real -> int
val toInt : IEEEReal.rounding_mode -> real -> int
val toLargeInt : IEEEReal.rounding_mode -> real -> LargeInt.int
val fromInt : int -> real
val fromLargeInt : LargeInt.int -> real
val toLarge : real -> LargeReal.real
val fromLarge : IEEEReal.rounding_mode -> LargeReal.real -> real
val fmt : StringCvt.realfmt -> real -> string
val toString : real -> string
val scan : (char, 'a) StringCvt.reader -> (real, 'a) StringCvt.reader
val fromString : string -> real option
val toDecimal : real -> IEEEReal.decimal_approx
val fromDecimal : IEEEReal.decimal_approx -> real option
```

#### Description

<span id="SIG:REAL.real:TY"></span>**`type`**` real`  
Note that, as discussed below, [`real`](real.md#SIG:REAL.real:TY:SPEC) is not an equality type.

<span id="SIG:REAL.radix:VAL"></span>**`val`**` radix `**`:`**` int`  
The base of the representation, _e.g._, 2 or 10 for IEEE floating point.

<span id="SIG:REAL.precision:VAL"></span>**`val`**` precision `**`:`**` int`  
The number of digits, each between `0` and [`radix`](real.md#SIG:REAL.radix:VAL:SPEC)`-1`, in the mantissa. Note that the precision includes the implicit (or hidden) bit used in the IEEE representation (_e.g._, the value of `Real64.precision` is `53`).

<span id="SIG:REAL.maxFinite:VAL"></span>**`val`**` maxFinite `**`:`**` real`
**`val`**` minPos `**`:`**` real`
**`val`**` minNormalPos `**`:`**` real`  
The maximum finite number, the minimum non-zero positive number, and the minimum non-zero normalized number, respectively.

<span id="SIG:REAL.posInf:VAL"></span>**`val`**` posInf `**`:`**` real`
**`val`**` negInf `**`:`**` real`  
Positive and negative infinity values.

<span id="SIG:REAL.+:VAL"></span>
`r1`` + ``r2`` `
` ``r1`` - ``r2`` `  
These denote the sum and difference of `r1` and `r2`. If one argument is finite and the other infinite, the result is infinite with the correct sign, _e.g._, 5 - (-infinity) = infinity. We also have infinity + infinity = infinity and (-infinity) + (-infinity) = (-infinity). Any other combination of two infinities produces NaN.

<span id="SIG:REAL.*:VAL"></span>
`r1`` * ``r2`` `  
denotes the product of `r1` and `r2`. The product of zero and an infinity produces NaN. Otherwise, if one argument is infinite, the result is infinite with the correct sign, _e.g._, -5 \* (-infinity) = infinity, infinity \* (-infinity) = -infinity.

<span id="SIG:REAL./:VAL"></span>
`r1`` / ``r2`` `  
denotes the quotient of `r1` and `r2`. We have 0 / 0 = `NaN` and +-infinity / +-infinity = `NaN`. Dividing a finite, non-zero number by a zero, or an infinity by a finite number produces an infinity with the correct sign. (Note that zeros are signed.) A finite number divided by an infinity is 0 with the correct sign.

<span id="SIG:REAL.rem:VAL"></span>
`rem (``x``, ``y``) `  
returns the remainder `x` - `n`\*`y`, where `n` = `trunc` (`x` / `y`). The result has the same sign as `x` and has absolute value less than the absolute value of `y`.

If `x` is an infinity or `y` is 0, [`rem`](real.md#SIG:REAL.rem:VAL:SPEC) returns NaN. If `y` is an infinity, [`rem`](real.md#SIG:REAL.rem:VAL:SPEC) returns `x`.

<span id="SIG:REAL.*+:VAL"></span>
`*+ (``a``, ``b``, ``c``) `
`*- (``a``, ``b``, ``c``)`  
These return `a``*``b`` + ``c` and `a``*``b`` - ``c`, respectively. Their behaviors on infinities follow from the behaviors derived from addition, subtraction, and multiplication.

The precise semantics of these operations depend on the language implementation and the underlying hardware. Specifically, certain architectures provide these operations as a single instruction, possibly using a single rounding operation. Thus, the use of these operations may be faster than performing the individual arithmetic operations sequentially, but may also cause different rounding behavior.

<span id="SIG:REAL.~:VAL"></span>
`~ ``r`` `  
produces the negation of `r`. `~` (+-infinity) = -+infinity.

<span id="SIG:REAL.abs:VAL"></span>
`abs ``r`` `  
returns the absolute value \|`r`\| of `r`.

> `abs` (+-0.0) = +0.0 `abs` (+-infinity) = +infinity `abs` (+-NaN) = +NaN


<span id="SIG:REAL.min:VAL"></span>**`val`**` min `**`:`**` real `**`*`**` real `**`->`**` real`
**`val`**` max `**`:`**` real `**`*`**` real `**`->`**` real`  
These return the smaller (respectively, larger) of the arguments. If exactly one argument is NaN, they return the other argument. If both arguments are NaN, they return NaN.

<span id="SIG:REAL.sign:VAL"></span>
`sign ``r`` `  
returns ~1 if `r` is negative, 0 if `r` is zero, or 1 if `r` is positive. An infinity returns its sign; a zero returns 0 regardless of its sign. It raises [`Domain`](general.md#SIG:GENERAL.Domain:EXN:SPEC) on NaN.

<span id="SIG:REAL.signBit:VAL"></span>
`signBit ``r`` `  
returns `true` if and only if the sign of `r` (infinities, zeros, and NaN, included) is negative.

<span id="SIG:REAL.sameSign:VAL"></span>
`sameSign (``r1``, ``r2``) `  
returns `true` if and only if [`signBit`](real.md#SIG:REAL.signBit:VAL:SPEC)` ``r1` equals [`signBit`](real.md#SIG:REAL.signBit:VAL:SPEC)` ``r2`.

<span id="SIG:REAL.copySign:VAL"></span>
`copySign (``x``, ``y``) `  
returns `x` with the sign of `y`, even if `y` is NaN.

<span id="SIG:REAL.compare:VAL"></span>**`val`**` compare `**`:`**` real `**`*`**` real `**`->`**` order`
**`val`**` compareReal `**`:`**` real `**`*`**` real `**`->`**` IEEEReal.real_order`  
The function [`compare`](real.md#SIG:REAL.compare:VAL:SPEC) returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) according to whether its first argument is less than, equal to, or greater than the second. It raises [`IEEEReal.Unordered`](ieee-float.md#SIG:IEEE_REAL.Unordered:EXN:SPEC) on unordered arguments.

The function [`compareReal`](real.md#SIG:REAL.compareReal:VAL:SPEC) behaves similarly except that the values it returns have the extended type [`IEEEReal.real_order`](ieee-float.md#SIG:IEEE_REAL.real_order:TY:SPEC) and it returns [`IEEEReal.UNORDERED`](ieee-float.md#SIG:IEEE_REAL.real_order:TY:SPEC) on unordered arguments.

> **Implementation note:**
>
> Implementations should try to optimize use of `compare`, since it is necessary for catching NaNs.


<span id="SIG:REAL.\|@LT\|:VAL"></span>**`val`**` < `**`:`**` real `**`*`**` real `**`->`**` bool`
**`val`**` <= `**`:`**` real `**`*`**` real `**`->`**` bool`
**`val`**` > `**`:`**` real `**`*`**` real `**`->`**` bool`
**`val`**` >= `**`:`**` real `**`*`**` real `**`->`**` bool`  
These return `true` if the corresponding relation holds between the two reals.

Note that these operators return `false` on unordered arguments, _i.e._, if either argument is NaN, so that the usual reversal of comparison under negation does not hold, _e.g._, `a < b` is not the same as `not (a >= b)`.

<span id="SIG:REAL.==:VAL"></span>
`== (``x``, ``y``) `
`!= (``x``, ``y``)`  
The first returns `true` if and only if neither `y` nor `x` is NaN, and `y` and `x` are equal, ignoring signs on zeros. This is equivalent to the IEEE `=` operator.

The second function `!=` is equivalent to `not o op ==` and the IEEE `?<>` operator.

<span id="SIG:REAL.?=:VAL"></span>**`val`**` ?= `**`:`**` real `**`*`**` real `**`->`**` bool`  
This returns `true` if either argument is NaN or if the arguments are bitwise equal, ignoring signs on zeros. It is equivalent to the IEEE `?=` operator.

<span id="SIG:REAL.unordered:VAL"></span>
`unordered (``x``, ``y``) `  
returns `true` if `x` and `y` are unordered, _i.e._, at least one of `x` and `y` is NaN.

<span id="SIG:REAL.isFinite:VAL"></span>
`isFinite ``x`` `  
returns `true` if `x` is neither NaN nor an infinity.

<span id="SIG:REAL.isNan:VAL"></span>
`isNan ``x`` `  
returns `true` if `x` is NaN.

<span id="SIG:REAL.isNormal:VAL"></span>
`isNormal ``x`` `  
returns `true` if `x` is normal, _i.e._, neither zero, subnormal, infinite nor NaN.

<span id="SIG:REAL.class:VAL"></span>
`class ``x`` `  
returns the [`IEEEReal.float_class`](ieee-float.md#SIG:IEEE_REAL.float_class:TY:SPEC) to which `x` belongs.

<span id="SIG:REAL.toManExp:VAL"></span>
`toManExp ``r`` `  
returns `{``man``, ``exp``}`, where `man` and `exp` are the mantissa and exponent of `r`, respectively. Specifically, we have the relation

> `r` = `man` \* `radix`<sup>(`exp`)</sup>

where 1.0 \<= `man` \* `radix` \< `radix`. This function is comparable to `frexp` in the C library.

If `r` is +-0, `man` is +-0 and `exp` is +0. If `r` is +-infinity, `man` is +-infinity and `exp` is unspecified. If `r` is NaN, `man` is NaN and `exp` is unspecified.

<span id="SIG:REAL.fromManExp:VAL"></span>
`fromManExp {``man``, ``exp``} `  
returns `man` \* `radix`<sup>(`exp`)</sup>. This function is comparable to `ldexp` in the C library. Note that, even if `man` is a non-zero, finite real value, the result of [`fromManExp`](real.md#SIG:REAL.fromManExp:VAL:SPEC) can be zero or infinity because of underflows and overflows.

If `man` is +-0, the result is +-0. If `man` is +-infinity, the result is +-infinity. If `man` is NaN, the result is NaN.

<span id="SIG:REAL.split:VAL"></span>
`split ``r`` `
` realMod ``r`` `  
The former returns `{``whole``, ``frac``}`, where `frac` and `whole` are the fractional and integral parts of `r`, respectively. Specifically, `whole` is integral, \|`frac`\| \< 1.0, `whole` and `frac` have the same sign as `r`, and `r` = `whole` + `frac`. This function is comparable to `modf` in the C library.

If `r` is +-infinity, `whole` is +-infinity and `frac` is +-0. If `r` is NaN, both `whole` and `frac` are NaN.

`realMod` is equivalent to `#frac o split`.

<span id="SIG:REAL.nextAfter:VAL"></span>
`nextAfter (``r``, ``t``) `  
returns the next representable real after `r` in the direction of `t`. Thus, if `t` is less than `r`, [`nextAfter`](real.md#SIG:REAL.nextAfter:VAL:SPEC) returns the largest representable floating-point number less than `r`. If `r`` = ``t` then it returns `r`. If either argument is NaN, this returns NaN. If `r` is +-infinity, it returns +-infinity.

<span id="SIG:REAL.checkFloat:VAL"></span>
`checkFloat ``x`` `  
raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if `x` is an infinity, and raises [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) if `x` is NaN. Otherwise, it returns its argument.

This can be used to synthesize trapping arithmetic from the non-trapping operations given here. Note, however, that infinities can be converted to NaNs by some operations, so that if accurate exceptions are required, checks must be done after each operation.

<span id="SIG:REAL.realFloor:VAL"></span>
`realFloor ``r`` `
` realCeil ``r`` `
` realTrunc ``r`` `
` realRound ``r`` `  
These functions convert real values to integer-valued reals. [`realFloor`](real.md#SIG:REAL.realFloor:VAL:SPEC) produces **floor**(r), the largest integer not larger than `r`. [`realCeil`](real.md#SIG:REAL.realCeil:VAL:SPEC) produces **ceil**(r), the smallest integer not less than `r`. [`realTrunc`](real.md#SIG:REAL.realTrunc:VAL:SPEC) rounds `r` towards zero, and [`realRound`](real.md#SIG:REAL.realRound:VAL:SPEC) rounds to the integer-values real value that is _nearest_ to `r`. If `r` is NaN or an infinity, these functions return `r`.

<span id="SIG:REAL.floor:VAL"></span>
`floor ``r`` `
` ceil ``r`` `
` trunc ``r`` `
` round ``r`` `  
These functions convert reals to integers. [`floor`](real.md#SIG:REAL.floor:VAL:SPEC) produces **floor**(r), the largest [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) not larger than `r`. [`ceil`](real.md#SIG:REAL.ceil:VAL:SPEC) produces **ceil**(r), the smallest [`int`](integer.md#SIG:INTEGER.int:TY:SPEC) not less than `r`. [`trunc`](real.md#SIG:REAL.trunc:VAL:SPEC) rounds `r` towards zero. [`round`](real.md#SIG:REAL.round:VAL:SPEC) yields the integer nearest to `r`. In the case of a tie, it rounds to the nearest even integer. They raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the resulting value cannot be represented as an [`int`](integer.md#SIG:INTEGER.int:TY:SPEC), for example, on infinity. They raise [`Domain`](general.md#SIG:GENERAL.Domain:EXN:SPEC) on NaN arguments.

These are respectively equivalent to:

toInt IEEEReal.TO_NEGINF r
toInt IEEEReal.TO_POSINF r
toInt IEEEReal.TO_ZERO r
toInt IEEEReal.TO_NEAREST r


<span id="SIG:REAL.toInt:VAL"></span>
`toInt ``mode`` ``x`` `
` toLargeInt ``mode`` ``x`` `  
These functions convert the argument `x` to an integral type using the specified rounding mode. They raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the result is not representable, in particular, if `x` is an infinity. They raise [`Domain`](general.md#SIG:GENERAL.Domain:EXN:SPEC) if the input real is NaN.

<span id="SIG:REAL.fromInt:VAL"></span>
`fromInt ``i`` `
` fromLargeInt ``i`` `  
These functions convert the integer `i` to a [`real`](real.md#SIG:REAL.real:TY:SPEC) value. If the absolute value of `i` is larger than [`maxFinite`](real.md#SIG:REAL.maxFinite:VAL:SPEC), then the appropriate infinity is returned. If `i` cannot be exactly represented as a [`real`](real.md#SIG:REAL.real:TY:SPEC) value, then the current rounding mode is used to determine the resulting value. The top-level function `real` is an alias for [`Real.fromInt`](real.md#SIG:REAL.fromInt:VAL:SPEC).

<span id="SIG:REAL.toLarge:VAL"></span>
`toLarge ``r`` `
` fromLarge ``r`` `  
These convert between values of type [`real`](real.md#SIG:REAL.real:TY:SPEC) and type [`LargeReal.real`](real.md#SIG:REAL.real:TY:SPEC). If `r` is too small or too large to be represented as a [`real`](real.md#SIG:REAL.real:TY:SPEC), [`fromLarge`](real.md#SIG:REAL.fromLarge:VAL:SPEC) will convert it to a zero or an infinity.

<span id="SIG:REAL.fmt:VAL"></span>
`fmt ``spec`` ``r`` `
` toString ``r`` `  
These functions convert reals into strings. The conversion provided by the function [`fmt`](real.md#SIG:REAL.fmt:VAL:SPEC) is parameterized by `spec`, which has the following forms and interpretations.

`SCI ``arg`  
Scientific notation:

> \[`~`\]<sup>?</sup>\[`0`-`9`\]`.`\[`0`-`9`\]<sup>+?</sup>`E`\[`0`-`9`\]<sup>+</sup>

where there is always one digit before the decimal point, nonzero if the number is nonzero. `arg` specifies the number of digits to appear after the decimal point, with 6 the default if `arg` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). If `arg` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(0)`, no fractional digits and no decimal point are printed.

`FIX ``arg`  
Fixed-point notation:

> \[`~`\]<sup>?</sup>\[`0`-`9`\]<sup>+</sup>`.`\[`0`-`9`\]<sup>+?</sup>

`arg` specifies the number of digits to appear after the decimal point, with 6 the default if `arg` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). If `arg` is [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(0)`, no fractional digits and no decimal point are printed.

`GEN ``arg`  
Adaptive notation: the notation used is either scientific or fixed-point depending on the value converted. `arg` specifies the maximum number of significant digits used, with 12 the default if `arg` is [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

`EXACT`  
Exact decimal notation: refer to [`IEEEReal.toString`](ieee-float.md#SIG:IEEE_REAL.toString:VAL:SPEC) for a complete description of this format.

In all cases, positive and negative infinities are converted to `"inf"` and `"~inf"`, respectively, and NaN values are converted to the string `"nan"`.

Refer to [`StringCvt.realfmt`](string-cvt.md#SIG:STRING_CVT.realfmt:TY:SPEC) for more details concerning these formats, especially the adaptive format `GEN`.

[`fmt`](real.md#SIG:REAL.fmt:VAL:SPEC) raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if `spec` is an invalid precision, _i.e._, if `spec` is

- [`SCI`](string-cvt.md#SIG:STRING_CVT.realfmt:TY:SPEC)` (`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``i``)` with `i` \< 0
- [`FIX`](string-cvt.md#SIG:STRING_CVT.realfmt:TY:SPEC)` (`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``i``)` with `i` \< 0
- [`GEN`](string-cvt.md#SIG:STRING_CVT.realfmt:TY:SPEC)` (`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``i``)` with `i` \< 1

The exception should be raised when `fmt ``spec` is evaluated.

The [`fmt`](real.md#SIG:REAL.fmt:VAL:SPEC) function allows the user precise control as to the form of the resulting string. Note, therefore, that it is possible for [`fmt`](real.md#SIG:REAL.fmt:VAL:SPEC) to produce a result that is not a valid SML string representation of a real value.

The value returned by [`toString`](real.md#SIG:REAL.toString:VAL:SPEC) is equivalent to:

([fmt](real.md#SIG:REAL.fmt:VAL:SPEC) ([StringCvt.GEN](string-cvt.md#SIG:STRING_CVT.realfmt:TY:SPEC) [NONE](option.md#SIG:OPTION.option:TY:SPEC)) `r`)


<span id="SIG:REAL.scan:VAL"></span>
`scan ``getc`` ``strm`` `
` fromString ``s`` `  
These functions scan a [`real`](real.md#SIG:REAL.real:TY:SPEC) value from character source. The first version reads from ARG/strm/ using reader `getc`, ignoring initial whitespace. It returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``r``,``rest``)` if successful, where `r` is the scanned [`real`](real.md#SIG:REAL.real:TY:SPEC) value and `rest` is the unused portion of the character stream `strm`. Values of too large a magnitude are represented as infinities; values of too small a magnitude are represented as zeros.

The second version returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``r``)` if a [`real`](real.md#SIG:REAL.real:TY:SPEC) value can be scanned from a prefix of `s`, ignoring any initial whitespace; otherwise, it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). This function is equivalent to [`StringCvt.scanString`](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC)` scan`.

The functions accept real numbers with the following format:

> \[`+~-`\]<sup>?</sup>(\[`0`-`9`\]<sup>+</sup>`.`\[`0`-`9`\]<sup>+?</sup> \| `.`\[`0`-`9`\]<sup>+</sup>)(`e` \| `E`)\[`+~-`\]<sup>?</sup>\[`0`-`9`\]<sup>+?</sup>

It also accepts the following string representations of non-finite values:

> \[`+~-`\]<sup>?</sup>(`inf` \| `infinity` \| `nan`)

where the alphabetic characters are case-insensitive.

<span id="SIG:REAL.toDecimal:VAL"></span>
`toDecimal ``r`` `
` fromDecimal ``d`` `  
These convert between [`real`](real.md#SIG:REAL.real:TY:SPEC) values and decimal approximations. Decimal approximations are to be converted using the [`IEEEReal.TO_NEAREST`](ieee-float.md#SIG:IEEE_REAL.rounding_mode:TY:SPEC) rounding mode. `toDecimal` should produce only as many digits as are necessary for `fromDecimal` to convert back to the same number. In particular, for any normal or subnormal real value `r`, we have the bit-wise equality:

fromDecimal (toDecimal `r`) = `r`.

For `toDecimal`, when the `r` is not normal or subnormal, then the `exp` field is set to 0 and the `digits` field is the empty list. In all cases, the `sign` and `class` field capture the sign and class of `r`.

For `fromDecimal`, if `class` is `ZERO` or `INF`, the resulting real is the appropriate signed zero or infinity. If `class` is `NAN`, a signed NaN is generated. If `class` is `NORMAL` or `SUBNORMAL`, the `sign`, `digits` and `exp` fields are used to produce a real number whose value is.

> s \* 0.d<sub>(1)</sub>d<sub>(2)</sub>...d<sub>(n)</sub> 10<sup>(exp)</sup>

where `digits` = \[d<sub>(1)</sub>, d<sub>(2)</sub>, ..., d<sub>(n)</sub>\] and where s is -1 if `sign` is `true` and 1 otherwise. Note that the conversion itself should ignore the `class` field, so that the resulting value might have class `NORMAL`, `SUBNORMAL`, `ZERO`, or `INF`. For example, if `digits` is empty or a list of all 0's, the result should be a signed zero. More generally, very large or small magnitudes are converted to infinities or zeros.

If the argument to `fromDecimal` does not have a valid format, _i.e._, if the `digits` field contains integers outside the range \[0,9\], it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC).

> **Implementation note:**
>
> Algorithms for accurately and efficiently converting between binary and decimal real representations are readily available, _e.g._, see the technical report by Gay**\[CITE\]**.


#### Examples

```repl
Real.floor (Real.fromInt 3);;
```

#### See Also

> [`IEEEReal`](ieee-float.md#IEEEReal:STR:SPEC), [`MATH`](math.md#MATH:SIG:SPEC), [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC)

#### Discussion

If [`LargeReal`](real.md#LargeReal:STR:SPEC) is not the same as [`Real`](real.md#Real:STR:SPEC), then there must be a structure [`Real`_`<N>`_](real.md#Real%7BN%7D:STR:SPEC) equal to [`LargeReal`](real.md#LargeReal:STR:SPEC).

The sign of a zero is ignored in all comparisons.

Unless specified otherwise, any operation involving NaN will return NaN.

Note that, if `x` is real, `~``x` is equivalent to `~(``x``)`, that is, it is identical to `x` but with its sign bit flipped. In particular, the literal `~0.0` is just `0.0` with its sign bit set. On the other hand, this might not be the same as `0.0-0.0`, in which rounding modes come into play.

Except for the `*+` and `*-` functions, arithmetic should be done in the exact precision specified by the [`precision`](real.md#SIG:REAL.precision:VAL:SPEC) value. In particular, arithmetic must not be done in some extended precision and then rounded.

The relation between the comparison predicates defined here and those defined by IEEE, ANSI C, and FORTRAN is specified in the following table.

---

**SML**

**IEEE**

**C**

**FORTRAN**

`==`

`=`

`==`

`.EQ.`

`!=`

`?<>`

`!=`

`.NE.`

`<`

`<`

`<`

`.LT.`

`<=`

`<=`

`<=`

`.LE.`

`>`

`>`

`>`

`.GT.`

`>=`

`>=`

`>=`

`.GE.`

`?=`

`?=`

`!islessgreater`

`.UE.`

`not o ?=`

`<>`

`islessgreater`

`.LG.`

`unordered`

`?`

`isunordered`

`unordered`

`not o unordered`

`<=>`

`!isunordered`

`.LEG.`

`not o op <`

`?>=`

`! <`

`.UGE.`

`not o op <=`

`?>`

`! <=`

`.UG.`

`not o op >`

`?<=`

`! >`

`.ULE.`

`not o op >=`

`?<`

`! >=`

`.UL.`

---

> **Implementation note:**
>
> Implementations may choose to provide a debugging mode, in which NaNs and infinities are detected when they are generated.

> **Rationale:**
>
> The specification of the default signature and structure for non-integer arithmetic, particularly concerning exceptional conditions, was the source of much debate, given the desire of supporting efficient floating-point modules. If we permit implementations to differ on whether or not, for example, to raise [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) on division by zero, the user really would not have a standard to program against. Portable code would require adopting the more conservative position of explicitly handling exceptions. A second alternative was to specify that functions in the `Real` structure must raise exceptions, but that implementations so desiring could provide additional structures matching `REAL` with explicit floating-point semantics. This was rejected because it meant that the default `real` type would not be the same as a defined floating-point `real` type. This would give a second-class status to the latter, while providing the default real with worse performance and involving additional implementation complexity for little benefit.
>
> Deciding if `real` should be an equality type, and if so, what should equality mean, was also problematic. IEEE specifies that the sign of zeros be ignored in comparisons, and that equality evaluate to false if either argument is NaN. These constraints are disturbing to the SML programmer. The former implies that `0 = ~0` is true while `r/0 = r/~0` is false. The latter implies such anomalies as `r = r` is false, or that, for a ref cell `rr`, we could have `rr = rr` but not have `!rr = !rr`. We accepted the unsigned comparison of zeros, but felt that the reflexive property of equality, structural equality, and the equivalence of `<>` and `not o =` ought to be preserved. Additional complications led to the decision to not have `real` be an equality type.
>
> The type, signature, and structure identifiers `real`, `REAL`, and `Real`, although misnomers in light of the floating-point-specific nature of the modules, were retained for historical reasons.
