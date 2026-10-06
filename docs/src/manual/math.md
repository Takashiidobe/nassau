# <span id="section:0"></span>The `MATH` signature

---

#### Synopsis

<span id="MATH:SIG:SPEC"></span>
<span id="Math:STR:SPEC"></span>

```sml
signature MATH
structure Math :> MATH
where type real = Real.real
```

The signature `MATH` specifies basic mathematical constants, the square root function, and trigonometric, hyperbolic, exponential, and logarithmic functions based on a real type. The functions defined here have roughly the same semantics as their counterparts in ISO C's `math.h`.

The top-level structure `Math` provides these functions for the default real type [`Real.real`](real.md#SIG:REAL.real:TY:SPEC).

In the functions below, unless specified otherwise, if any argument is a NaN, the return value is a NaN. In a list of rules specifying the behavior of a function in special cases, the first matching rule defines the semantics.

---

#### Interface

<span id="SIG:MATH.real:TY:SPEC"></span>
<span id="SIG:MATH.pi:VAL:SPEC"></span>
<span id="SIG:MATH.e:VAL:SPEC"></span>
<span id="SIG:MATH.sqrt:VAL:SPEC"></span>
<span id="SIG:MATH.sin:VAL:SPEC"></span>
<span id="SIG:MATH.cos:VAL:SPEC"></span>
<span id="SIG:MATH.tan:VAL:SPEC"></span>
<span id="SIG:MATH.asin:VAL:SPEC"></span>
<span id="SIG:MATH.acos:VAL:SPEC"></span>
<span id="SIG:MATH.atan:VAL:SPEC"></span>
<span id="SIG:MATH.atan2:VAL:SPEC"></span>
<span id="SIG:MATH.exp:VAL:SPEC"></span>
<span id="SIG:MATH.pow:VAL:SPEC"></span>
<span id="SIG:MATH.ln:VAL:SPEC"></span>
<span id="SIG:MATH.log10:VAL:SPEC"></span>
<span id="SIG:MATH.sinh:VAL:SPEC"></span>
<span id="SIG:MATH.cosh:VAL:SPEC"></span>
<span id="SIG:MATH.tanh:VAL:SPEC"></span>

```sml
type real
val pi : real
val e : real
val sqrt : real -> real
val sin : real -> real
val cos : real -> real
val tan : real -> real
val asin : real -> real
val acos : real -> real
val atan : real -> real
val atan2 : real * real -> real
val exp : real -> real
val pow : real * real -> real
val ln : real -> real
val log10 : real -> real
val sinh : real -> real
val cosh : real -> real
val tanh : real -> real
```

#### Description

<span id="SIG:MATH.pi:VAL"></span>

### `pi`

```sml
val pi : real
```
The constant pi (3.141592653...).


```repl
Math.pi;; (* approximately 3.14159 *)
```

<span id="SIG:MATH.e:VAL"></span>

### `e`

```sml
val e : real
```
The base `e` (2.718281828...) of the natural logarithm.


```repl
Math.e;; (* approximately 2.71828 *)
```

<span id="SIG:MATH.sqrt:VAL"></span>

### `sqrt`

```sml
val sqrt : real -> real
```
returns the square root of `x`. [`sqrt`](math.md#SIG:MATH.sqrt:VAL:SPEC)` (~0.0) = ~0.0`. If `x` \< 0, it returns NaN.


```repl
Math.sqrt 4.0;; (* 2.0 *)
Math.sqrt (~1.0);; (* NaN *)
```

<span id="SIG:MATH.sin:VAL"></span>

### `sin`

```sml
val sin : real -> real
```

### `cos`

```sml
val cos : real -> real
```

### `tan`

```sml
val tan : real -> real
```
These return the sine, cosine, and tangent, respectively, of `x`, measured in radians. If `x` is an infinity, these functions return NaN. Note that [`tan`](math.md#SIG:MATH.tan:VAL:SPEC) will produce infinities at various finite values, roughly corresponding to the singularities of the tangent function.


```repl
Math.sin 0.0;; (* 0.0 *)
Math.cos 0.0;; (* 1.0 *)
Math.tan 0.0;; (* 0.0 *)
```

<span id="SIG:MATH.asin:VAL"></span>

### `asin`

```sml
val asin : real -> real
```

### `acos`

```sml
val acos : real -> real
```
These return the arc sine and arc cosine, respectively, of `x`. [`asin`](math.md#SIG:MATH.asin:VAL:SPEC) is the inverse of [`sin`](math.md#SIG:MATH.sin:VAL:SPEC). Its result is guaranteed to be in the closed interval \[-pi/2,pi/2\]. [`acos`](math.md#SIG:MATH.acos:VAL:SPEC) is the inverse of [`cos`](math.md#SIG:MATH.cos:VAL:SPEC). Its result is guaranteed to be in the closed interval \[0,pi\]. If the magnitude of `x` exceeds 1.0, they return NaN.


```repl
Math.asin 0.0;; (* 0.0 *)
Math.acos 1.0;; (* 0.0 *)
```

<span id="SIG:MATH.atan:VAL"></span>

### `atan`

```sml
val atan : real -> real
```
returns the arc tangent of `x`. [`atan`](math.md#SIG:MATH.atan:VAL:SPEC) is the inverse of [`tan`](math.md#SIG:MATH.tan:VAL:SPEC). For finite arguments, the result is guaranteed to be in the open interval (-pi/2,pi/2). If `x` is +infinity, it returns pi/2; if `x` is -infinity, it returns -pi/2.


```repl
Math.atan 0.0;; (* 0.0 *)
```

<span id="SIG:MATH.atan2:VAL"></span>

### `atan2`

```sml
val atan2 : real * real -> real
```
returns the arc tangent of `(``y``/``x``)` in the closed interval \[-pi,pi\], corresponding to angles within +-180 degrees. The quadrant of the resulting angle is determined using the signs of both `x` and `y`, and is the same as the quadrant of the point (`x`,`y`). When `x` = 0, this corresponds to an angle of 90 degrees, and the result is `(real (sign ``y``)) * pi/2.0`. It holds that

> `sign` ( `cos` ( `atan2` (`y`,`x`))) = `sign`(`x`)

and

> `sign` ( `sin` ( `atan2` (`y`,`x`))) = `sign`(`y`)

except for inaccuracies incurred by the finite precision of [`real`](math.md#SIG:MATH.real:TY:SPEC) and the approximation algorithms used to compute the mathematical functions.

Rules for exceptional cases are specified in the following table.

---

`y`

`x`

`atan2(``y``,``x``)`

+-0

0 \< `x`

+-0

+-0

+0

+-0

+-0

`x` \< 0

+-pi

+-0

-0

+-pi

`y`, 0 \< `y`

+-0

pi/2

`y`, `y` \< 0

+-0

-pi/2

+-`y`, finite `y` \> 0

+infinity

+-0

+-`y`, finite `y` \> 0

-infinity

+-pi

+-infinity

finite `x`

+-pi/2

+-infinity

+infinity

+-pi/4

+-infinity

-infinity

+-3pi/4

---



```repl
Math.atan2 (0.0, 1.0);; (* 0.0 *)
```

<span id="SIG:MATH.exp:VAL"></span>

### `exp`

```sml
val exp : real -> real
```
returns e<sup>(`x`)</sup>, _i.e._, e raised to the `x`<sup>(th)</sup> power. If `x` is +infinity, it returns +infinity; if `x` is -infinity, it returns 0.


```repl
Math.exp 0.0;; (* 1.0 *)
```

<span id="SIG:MATH.pow:VAL"></span>

### `pow`

```sml
val pow : real * real -> real
```
returns `x`<sup>(`y`)</sup>, _i.e._, `x` raised to the `y`<sup>(th)</sup> power. For finite `x` and `y`, this is well-defined when `x` \> 0, or when `x` \< 0 and `y` is integral. Rules for exceptional cases are specified below.

---

`x`

`y`

`pow(``x``,``y``)`

`x`, including NaN

0

1

\|`x`\| \> 1

+infinity

+infinity

\|`x`\| \< 1

+infinity

+0

\|`x`\| \> 1

-infinity

+0

\|`x`\| \< 1

-infinity

+infinity

+infinity

`y` \> 0

+infinity

+infinity

`y` \< 0

+0

-infinity

`y` \> 0, odd integer

-infinity

-infinity

`y` \> 0, not odd integer

+infinity

-infinity

`y` \< 0, odd integer

-0

-infinity

`y` \< 0, not odd integer

+0

`x`

NaN

NaN

NaN

`y` \<\> 0

NaN

+-1

+-infinity

NaN

finite `x` \< 0

finite non-integer `y`

NaN

+-0

`y` \< 0, odd integer

+-infinity

+-0

finite `y` \< 0, not odd integer

+infinity

+-0

`y` \> 0, odd integer

+-0

+-0

`y` \> 0, not odd integer

+0

---



```repl
Math.pow (2.0, 3.0);; (* 8.0 *)
```

<span id="SIG:MATH.ln:VAL"></span>

### `ln`

```sml
val ln : real -> real
```

### `log10`

```sml
val log10 : real -> real
```
These return the natural logarithm (base e) and decimal logarithm (base 10), respectively, of `x`. If `x` \< 0, they return NaN; if `x` = 0, they return -infinity; if `x` is infinity, they return infinity.


```repl
Math.ln 1.0;; (* 0.0 *)
Math.log10 100.0;; (* 2.0 *)
```

<span id="SIG:MATH.sinh:VAL"></span>

### `sinh`

```sml
val sinh : real -> real
```

### `cosh`

```sml
val cosh : real -> real
```

### `tanh`

```sml
val tanh : real -> real
```
These return the hyperbolic sine, hyperbolic cosine, and hyperbolic tangent, respectively, of `x`, that is, the values (e<sup>(`x`)</sup> - e<sup>(-`x`)</sup>) / 2, (e<sup>(`x`)</sup> + e<sup>(-`x`)</sup>) / 2, and `(`[`sinh`](math.md#SIG:MATH.sinh:VAL:SPEC)` x)/(`[`cosh`](math.md#SIG:MATH.cosh:VAL:SPEC)` x)`.

These functions have the following properties:

---

[`sinh`](math.md#SIG:MATH.sinh:VAL:SPEC) +-0

=

+-0

[`sinh`](math.md#SIG:MATH.sinh:VAL:SPEC) +-infinity

=

+-infinity

[`cosh`](math.md#SIG:MATH.cosh:VAL:SPEC) +-0

=

1

[`cosh`](math.md#SIG:MATH.cosh:VAL:SPEC) +-infinity

=

+-infinity

[`tanh`](math.md#SIG:MATH.tanh:VAL:SPEC) +-0

=

+-0

[`tanh`](math.md#SIG:MATH.tanh:VAL:SPEC) +-infinity

=

+-1

---


```repl
Math.sinh 0.0;; (* 0.0 *)
Math.cosh 0.0;; (* 1.0 *)
Math.tanh 0.0;; (* 0.0 *)
```


#### See Also

> [`REAL`](real.md#REAL:SIG:SPEC)
