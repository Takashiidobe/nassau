# <span id="section:0"></span>The `Time` structure

---

#### Synopsis

<span id="TIME:SIG:SPEC"></span>
<span id="Time:STR:SPEC"></span>

```sml
signature TIME
structure Time :> TIME
```

The structure `Time` provides an abstract type for representing times and time intervals, and functions for manipulating, converting, writing, and reading them.

---

#### Interface

<span id="SIG:TIME.time:TY:SPEC"></span>
<span id="SIG:TIME.Time:EXN:SPEC"></span>
<span id="SIG:TIME.zeroTime:VAL:SPEC"></span>
<span id="SIG:TIME.fromReal:VAL:SPEC"></span>
<span id="SIG:TIME.toReal:VAL:SPEC"></span>
<span id="SIG:TIME.toSeconds:VAL:SPEC"></span>
<span id="SIG:TIME.toMilliseconds:VAL:SPEC"></span>
<span id="SIG:TIME.toMicroseconds:VAL:SPEC"></span>
<span id="SIG:TIME.toNanoseconds:VAL:SPEC"></span>
<span id="SIG:TIME.fromSeconds:VAL:SPEC"></span>
<span id="SIG:TIME.fromMilliseconds:VAL:SPEC"></span>
<span id="SIG:TIME.fromMicroseconds:VAL:SPEC"></span>
<span id="SIG:TIME.fromNanoseconds:VAL:SPEC"></span>
<span id="SIG:TIME.+:VAL:SPEC"></span>
<span id="SIG:TIME.-:VAL:SPEC"></span>
<span id="SIG:TIME.compare:VAL:SPEC"></span>
<span id="SIG:TIME.\|@LT\|:VAL:SPEC"></span>
<span id="SIG:TIME.\|@LTE\|:VAL:SPEC"></span>
<span id="SIG:TIME.\|@GT\|:VAL:SPEC"></span>
<span id="SIG:TIME.\|@GTE\|:VAL:SPEC"></span>
<span id="SIG:TIME.now:VAL:SPEC"></span>
<span id="SIG:TIME.fmt:VAL:SPEC"></span>
<span id="SIG:TIME.toString:VAL:SPEC"></span>
<span id="SIG:TIME.scan:VAL:SPEC"></span>
<span id="SIG:TIME.fromString:VAL:SPEC"></span>

```sml
eqtype time
exception Time
val zeroTime : time
val fromReal : LargeReal.real -> time
val toReal : time -> LargeReal.real
val toSeconds : time -> LargeInt.int
val toMilliseconds : time -> LargeInt.int
val toMicroseconds : time -> LargeInt.int
val toNanoseconds : time -> LargeInt.int
val fromSeconds : LargeInt.int -> time
val fromMilliseconds : LargeInt.int -> time
val fromMicroseconds : LargeInt.int -> time
val fromNanoseconds : LargeInt.int -> time
val + : time * time -> time
val - : time * time -> time
val compare : time * time -> order
val < : time * time -> bool
val <= : time * time -> bool
val > : time * time -> bool
val >= : time * time -> bool
val now : unit -> time
val fmt : int -> time -> string
val toString : time -> string
val scan : (char, 'a) StringCvt.reader -> (time, 'a) StringCvt.reader
val fromString : string -> time option
```

#### Description

<span id="SIG:TIME.time:TY"></span>**`eqtype`**` time`  
The type used to represent both absolute times and durations of time intervals, including negative values moving to the past. Absolute times are represented in the same way as time intervals, and can be thought of as time intervals starting at some fixed reference point. Their discrimination is only conceptual. Consequently, operations can be applied to all meaningful combinations (but also meaningless ones) of absolute times and intervals.

> **Implementation note:**
>
> Time values are required to have fixed-point semantics.


<span id="SIG:TIME.Time:EXN"></span>**`exception`**` Time`  
The exception raised when the result of conversions to [`time`](time.md#SIG:TIME.time:TY:SPEC) or of operations over [`time`](time.md#SIG:TIME.time:TY:SPEC) is not representable, or when an illegal operation has been attempted.

<span id="SIG:TIME.zeroTime:VAL"></span>

### `zeroTime`

```sml
val zeroTime : time
```
**`val`**` zeroTime `**`:`**` time`  
This denotes both the empty time interval and a common reference point for specifying absolute time values. It is equivalent to [`fromReal`](time.md#SIG:TIME.fromReal:VAL:SPEC)`(0.0)`.

Absolute points on the time scale can be thought of as being represented as intervals starting at [`zeroTime`](time.md#SIG:TIME.zeroTime:VAL:SPEC). The function [`Date.fromTimeLocal`](date.md#SIG:DATE.fromTimeLocal:VAL:SPEC) can be used to see what time [`zeroTime`](time.md#SIG:TIME.zeroTime:VAL:SPEC) actually represents in the local timezone.



```repl
Time.zeroTime;; (* 0 seconds *)
```

<span id="SIG:TIME.fromReal:VAL"></span>

### `fromReal`

```sml
val fromReal : LargeReal.real -> time
```

`fromReal ``r`` `  
converts the real number `r` to the time value denoting `r` seconds. Depending on the resolution of [`time`](time.md#SIG:TIME.time:TY:SPEC), fractions of a microsecond may be lost. It raises [`Time`](time.md#SIG:TIME.Time:EXN:SPEC) when the result is not representable.



```repl
Time.fromReal 1.5;; (* 1.5 seconds *)
```

<span id="SIG:TIME.toReal:VAL"></span>

### `toReal`

```sml
val toReal : time -> LargeReal.real
```

`toReal ``t`` `  
converts the time value `t` to a real number denoting the value of `t` in seconds. When the type [`real`](real.md#SIG:REAL.real:TY:SPEC) has less precision than [`Time.time`](time.md#SIG:TIME.time:TY:SPEC) (for example, when it is implemented as a single-precision float), information about microseconds or, for very large values, even seconds, may be lost.



```repl
Time.toReal (Time.fromSeconds 2);; (* 2.0 *)
```

<span id="SIG:TIME.toSeconds:VAL"></span>

### `toSeconds`

```sml
val toSeconds : time -> LargeInt.int
```

### `toMilliseconds`

```sml
val toMilliseconds : time -> LargeInt.int
```

### `toMicroseconds`

```sml
val toMicroseconds : time -> LargeInt.int
```

### `toNanoseconds`

```sml
val toNanoseconds : time -> LargeInt.int
```

`toSeconds ``t`` `
` toMilliseconds ``t`` `
` toMicroseconds ``t`` `
` toNanoseconds ``t`` `  
These functions return the number of full seconds (respectively, milliseconds, microseconds, or nanoseconds) in `t`; fractions of the time unit are dropped, _i.e._, the values are rounded towards 0. Thus, if `t` denotes 2.01 seconds, the functions return `2`, `2010`, `2010000`, and `2010000000` respectively. When the result is not representable by [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC), the exception [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) is raised.



```repl
Time.toSeconds (Time.fromReal 1.01);; (* 1 *)
```

```repl
Time.toMilliseconds (Time.fromReal 1.01);; (* 1010 *)
```

```repl
Time.toMicroseconds (Time.fromReal 1.01);; (* 1010000 *)
```

```repl
Time.toNanoseconds (Time.fromReal 1.01);; (* 1010000000 *)
```

<span id="SIG:TIME.fromSeconds:VAL"></span>

### `fromSeconds`

```sml
val fromSeconds : LargeInt.int -> time
```

### `fromMilliseconds`

```sml
val fromMilliseconds : LargeInt.int -> time
```

### `fromMicroseconds`

```sml
val fromMicroseconds : LargeInt.int -> time
```

### `fromNanoseconds`

```sml
val fromNanoseconds : LargeInt.int -> time
```

`fromSeconds ``n`` `
` fromMilliseconds ``n`` `
` fromMicroseconds ``n`` `
` fromNanoseconds ``n`` `  
These convert the number `n` to a time value denoting `n` seconds (respectively, milliseconds, microseconds, or nanoseconds). If the result is not representable by the [`time`](time.md#SIG:TIME.time:TY:SPEC) type, then the exception [`Time`](time.md#SIG:TIME.Time:EXN:SPEC) is raised.



```repl
Time.toSeconds (Time.fromSeconds 2);; (* 2 *)
```

```repl
Time.toReal (Time.fromMilliseconds 1500);; (* 1.5 *)
```

```repl
Time.toReal (Time.fromMicroseconds 1500000);; (* 1.5 *)
```

```repl
Time.toReal (Time.fromNanoseconds 1500000000);; (* 1.5 *)
```

<span id="SIG:TIME.+:VAL"></span>

### `+`

```sml
val + : time * time -> time
```

`t1`` + ``t2`` `  
returns a time interval denoting the duration of `t1` plus that of `t2`, when both `t1` and `t2` are interpreted as intervals. Equivalently, when `t1` is interpreted as an absolute time and `t2` as an interval, the absolute time that is `t2` later than `t1` is returned. (Both views are equivalent as absolute times are represented as intervals from [`zeroTime`](time.md#SIG:TIME.zeroTime:VAL:SPEC)). When the result is not representable as a time value, the exception [`Time`](time.md#SIG:TIME.Time:EXN:SPEC) is raised. This operation is commutative.



```repl
Time.toSeconds (Time.+ (Time.fromSeconds 2, Time.fromSeconds 3));; (* 5 *)
```

<span id="SIG:TIME.-:VAL"></span>

### `-`

```sml
val - : time * time -> time
```

`t1`` - ``t2`` `  
returns a time interval denoting the duration of `t1` minus that of `t2`, when both `t1` and `t2` are interpreted as intervals. Equivalently, when `t1` is interpreted as an absolute time and `t2` as an interval, the absolute time that is `t2` earlier than `t1` is returned; when both `t1` and `t2` are interpreted as absolute times, the interval between `t1` and `t2` is returned. (All views are equivalent as absolute times are represented as intervals from [`zeroTime`](time.md#SIG:TIME.zeroTime:VAL:SPEC)). When the result is not representable as a time value, the exception [`Time`](time.md#SIG:TIME.Time:EXN:SPEC) is raised.



```repl
Time.toSeconds (Time.- (Time.fromSeconds 5, Time.fromSeconds 2));; (* 3 *)
```

<span id="SIG:TIME.compare:VAL"></span>

### `compare`

```sml
val compare : time * time -> order
```

`compare (``t1``, ``t2``) `  
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) when the time interval `t1` is shorter than, of same length as, or longer than `t2`, respectively, or the absolute time `t1` is earlier than, coincides with, or is later than the absolute time `t2`.



```repl
Time.compare (Time.fromSeconds 1, Time.fromSeconds 2);; (* LESS *)
```

<span id="SIG:TIME.\|@LT\|:VAL"></span>

### `<`

```sml
val < : time * time -> bool
```

### `<=`

```sml
val <= : time * time -> bool
```

### `>`

```sml
val > : time * time -> bool
```

### `>=`

```sml
val >= : time * time -> bool
```
**`val`**` < `**`:`**` time `**`*`**` time `**`->`**` bool`
**`val`**` <= `**`:`**` time `**`*`**` time `**`->`**` bool`
**`val`**` > `**`:`**` time `**`*`**` time `**`->`**` bool`
**`val`**` >= `**`:`**` time `**`*`**` time `**`->`**` bool`  
These return `true` if the corresponding relation holds between the two times.



```repl
Time.< (Time.fromSeconds 1, Time.fromSeconds 2);; (* true *)
```

```repl
Time.<= (Time.fromSeconds 2, Time.fromSeconds 2);; (* true *)
```

```repl
Time.> (Time.fromSeconds 3, Time.fromSeconds 2);; (* true *)
```

```repl
Time.>= (Time.fromSeconds 3, Time.fromSeconds 2);; (* true *)
```

<span id="SIG:TIME.now:VAL"></span>

### `now`

```sml
val now : unit -> time
```
**`val`**` now `**`:`**` unit `**`->`**` time`  
The current time. This is usually interpreted as an absolute time, the time at which the function call was made. Although [`now`](time.md#SIG:TIME.now:VAL:SPEC) does not normally raise an exception, this may happen when it is called at a time that is not representable.



```repl
Time.toString (Time.now ());; (* current time, rounded to milliseconds *)
```

<span id="SIG:TIME.fmt:VAL"></span>

### `fmt`

```sml
val fmt : int -> time -> string
```

### `toString`

```sml
val toString : time -> string
```

`fmt ``n`` ``t`` `
` toString ``t`` `  
These return a string containing a decimal number representing `t` in seconds. Using [`fmt`](time.md#SIG:TIME.fmt:VAL:SPEC), the fractional part is rounded to `n` decimal digits. If `n` = 0, there should be no fractional part. Having `n` \< 0 causes the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception to be raised. [`toString`](time.md#SIG:TIME.toString:VAL:SPEC) rounds `t` to 3 decimal digits. It is equivalent to `fmt 3 ``t`.

> **Example:**
> fmt 3 (fromReal 1.8) = "1.800"
> fmt 0 (fromReal 1.8) = "2"
> fmt 0 zeroTime = "0"




```repl
Time.fmt 3 (Time.fromReal 1.8);; (* "1.800" *)
```

```repl
Time.toString (Time.fromReal 1.8);; (* "1.800" *)
```

<span id="SIG:TIME.scan:VAL"></span>

### `scan`

```sml
val scan : (char, 'a) StringCvt.reader -> (time, 'a) StringCvt.reader
```

### `fromString`

```sml
val fromString : string -> time option
```

`scan ``getc`` ``src`` `
` fromString ``s`` `  
These functions scan a time value from a character stream or a string. They recognize a number of seconds specified as a string that matches the regular expression:

> \[`+~-`\]<sup>?</sup>(\[`0`-`9`\]<sup>+</sup>`.`\[`0`-`9`\]<sup>+?</sup> \| `.`\[`0`-`9`\]<sup>+</sup>)

Initial whitespace is ignored. Both functions raise [`Time`](time.md#SIG:TIME.Time:EXN:SPEC) when the value is syntactically correct but not representable.

The function [`scan`](time.md#SIG:TIME.scan:VAL:SPEC) takes a character source `src` and an reader `getc` and tries to parse a time value from `src`. It returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``t``,``r``)` where `t` is the time value denoted by a prefix of `src` and `r` is the rest of `src`; or it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) when no prefix of `src` is a representation of a time value.

The function [`fromString`](time.md#SIG:TIME.fromString:VAL:SPEC) parses a time value from the string `s`, returning [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``t``)` where `t` is the time value denoted by a prefix of `s` or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) when no prefix of `s` is a representation of a time value. Note that this function is equivalent to [`StringCvt.scanString`](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC)` `[`scan`](time.md#SIG:TIME.scan:VAL:SPEC).



```repl
StringCvt.scanString Time.scan "2.5";; (* SOME 2.5 seconds *)
```

```repl
Time.fromString "2.5";; (* SOME 2.5 seconds *)
```

#### Examples

```repl
Time.fromString "2.5";; (* SOME 2.5 seconds *)
Time.fromString "";; (* NONE *)
```

#### See Also

> [`Date`](date.md#Date:STR:SPEC), [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC), [`Timer`](timer.md#Timer:STR:SPEC)
