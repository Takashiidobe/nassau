# <span id="section:0"></span>The `Date` structure

---

#### Synopsis

<span id="DATE:SIG:SPEC"></span>
<span id="Date:STR:SPEC"></span>

```sml
signature DATE
structure Date :> DATE
```

The `Date` structure provides functions for converting between times and dates, and formatting and scanning dates.

---

#### Interface

<span id="SIG:DATE.weekday:TY:SPEC"></span>
<span id="SIG:DATE.Mon:TY:SPEC"></span>
<span id="SIG:DATE.Tue:TY:SPEC"></span>
<span id="SIG:DATE.Wed:TY:SPEC"></span>
<span id="SIG:DATE.Thu:TY:SPEC"></span>
<span id="SIG:DATE.Fri:TY:SPEC"></span>
<span id="SIG:DATE.Sat:TY:SPEC"></span>
<span id="SIG:DATE.Sun:TY:SPEC"></span>
<span id="SIG:DATE.month:TY:SPEC"></span>
<span id="SIG:DATE.Jan:TY:SPEC"></span>
<span id="SIG:DATE.Feb:TY:SPEC"></span>
<span id="SIG:DATE.Mar:TY:SPEC"></span>
<span id="SIG:DATE.Apr:TY:SPEC"></span>
<span id="SIG:DATE.May:TY:SPEC"></span>
<span id="SIG:DATE.Jun:TY:SPEC"></span>
<span id="SIG:DATE.Jul:TY:SPEC"></span>
<span id="SIG:DATE.Aug:TY:SPEC"></span>
<span id="SIG:DATE.Sep:TY:SPEC"></span>
<span id="SIG:DATE.Oct:TY:SPEC"></span>
<span id="SIG:DATE.Nov:TY:SPEC"></span>
<span id="SIG:DATE.Dec:TY:SPEC"></span>
<span id="SIG:DATE.date:TY:SPEC"></span>
<span id="SIG:DATE.Date:EXN:SPEC"></span>
<span id="SIG:DATE.date:VAL:SPEC"></span>
<span id="SIG:DATE.year:VAL:SPEC"></span>
<span id="SIG:DATE.month:VAL:SPEC"></span>
<span id="SIG:DATE.day:VAL:SPEC"></span>
<span id="SIG:DATE.hour:VAL:SPEC"></span>
<span id="SIG:DATE.minute:VAL:SPEC"></span>
<span id="SIG:DATE.second:VAL:SPEC"></span>
<span id="SIG:DATE.weekDay:VAL:SPEC"></span>
<span id="SIG:DATE.yearDay:VAL:SPEC"></span>
<span id="SIG:DATE.offset:VAL:SPEC"></span>
<span id="SIG:DATE.isDst:VAL:SPEC"></span>
<span id="SIG:DATE.localOffset:VAL:SPEC"></span>
<span id="SIG:DATE.fromTimeLocal:VAL:SPEC"></span>
<span id="SIG:DATE.fromTimeUniv:VAL:SPEC"></span>
<span id="SIG:DATE.toTime:VAL:SPEC"></span>
<span id="SIG:DATE.compare:VAL:SPEC"></span>
<span id="SIG:DATE.fmt:VAL:SPEC"></span>
<span id="SIG:DATE.toString:VAL:SPEC"></span>
<span id="SIG:DATE.scan:VAL:SPEC"></span>
<span id="SIG:DATE.fromString:VAL:SPEC"></span>

```sml
datatype weekday = Mon | Tue | Wed | Thu | Fri | Sat | Sun
datatype month
= Jan
| Feb
| Mar
| Apr
| May
| Jun
| Jul
| Aug
| Sep
| Oct
| Nov
| Dec
type date
exception Date
val date : {
year : int,
month : month,
day : int,
hour : int,
minute : int,
second : int,
offset : Time.time option
} -> date
val year : date -> int
val month : date -> month
val day : date -> int
val hour : date -> int
val minute : date -> int
val second : date -> int
val weekDay : date -> weekday
val yearDay : date -> int
val offset : date -> Time.time option
val isDst : date -> bool option
val localOffset : unit -> Time.time
val fromTimeLocal : Time.time -> date
val fromTimeUniv : Time.time -> date
val toTime : date -> Time.time
val compare : date * date -> order
val fmt : string -> date -> string
val toString : date -> string
val scan : (char, 'a) StringCvt.reader -> (date, 'a) StringCvt.reader
val fromString : string -> date option
```

#### Description

<span id="SIG:DATE.date:TY"></span>**`type`**` date`  
An abstract type whose values represents an instant in a specific time zone.

<span id="SIG:DATE.date:VAL"></span>

### `date`

```sml
val date : {year : int, month : month, day : int, hour : int, minute : int, second : int, offset : Time.time option} -> date
```
creates a canonical date from the given date information. If the resulting date is outside the range supported by the implementation, the [`Date`](date.md#SIG:DATE.Date:EXN:SPEC) exception is raised.

Seconds outside the range \[0,59\] are converted to the equivalent minutes and added to the minutes argument. Similar conversions are performed for minutes to hours, hours to days, days to months, and months to years. Negative values are similarly translated into a canonical range, with the extra borrowed from the next larger unit. Thus, `minute = 10, second = ~140` becomes `minute = 7, second = 40`.

The `offset` argument provides time zone information. A value of [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) represents the local time zone. A value of [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``t``)` corresponds to time `t` west of UTC. In particular, [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(`[`Time.zeroTime`](time.md#SIG:TIME.zeroTime:VAL:SPEC)`)` is UTC. Negative offsets denote time zones to the east of UTC, as is traditional. Offsets are taken modulo 24 hours. That is, we express `t`, in hours, as sgn(`t`)(24\*`d` + `r`), where `d` and `r` are non-negative, `d` is integral, and `r` \< 24. The offset then becomes sgn(`t`)\*`r` and sgn(`t`)(24\*`d`) is added to the hours (before converting hours to days).

Leap years follow the Gregorian calendar. Leap seconds may or may not be ignored. In an implementation that takes account of leap seconds, the `second` function may return 60 or 61 in the rare cases that this is appropriate.


```repl
val d = Date.date {year = 2024, month = Date.Jan, day = 1, hour = 0, minute = 0, second = 0, offset = SOME Time.zeroTime};
Date.year d;; (* 2024 *)
```

<span id="SIG:DATE.year:VAL"></span>

### `year`

```sml
val year : date -> int
```

### `month`

```sml
val month : date -> month
```

### `day`

```sml
val day : date -> int
```

### `hour`

```sml
val hour : date -> int
```

### `minute`

```sml
val minute : date -> int
```

### `second`

```sml
val second : date -> int
```

### `weekDay`

```sml
val weekDay : date -> weekday
```

### `yearDay`

```sml
val yearDay : date -> int
```

### `offset`

```sml
val offset : date -> Time.time option
```

### `isDst`

```sml
val isDst : date -> bool option
```
These functions extract the attributes of a date value. The year returned by [`year`](date.md#SIG:DATE.year:VAL:SPEC) uses year 0 as its base. Thus, the date Robin Milnerreceived the Turing award would have year 1991. The function [`yearDay`](date.md#SIG:DATE.yearDay:VAL:SPEC) returns the day of the year, starting from 0, _i.e._, 1 January is day 0. The value returned by [`offset`](date.md#SIG:DATE.offset:VAL:SPEC) reports time zone information as the amount of time west of UTC. A value of [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) represents the local time zone. The function [`isDst`](date.md#SIG:DATE.isDst:VAL:SPEC) returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if the system has no information concerning daylight savings time. Otherwise, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``dst``)` where `dst` is `true` if daylight savings time is in effect.


```repl
val d = Date.date {year = 2024, month = Date.Jan, day = 1, hour = 0, minute = 0, second = 0, offset = SOME Time.zeroTime};
Date.year d;; (* 2024 *)
Date.month d;; (* Jan *)
Date.day d;; (* 1 *)
Date.hour d;; (* 0 *)
Date.minute d;; (* 0 *)
Date.second d;; (* 0 *)
Date.weekDay d;; (* Mon *)
Date.yearDay d;; (* 0 *)
Date.offset d;; (* SOME 0 *)
Date.isDst d;; (* SOME false or NONE *)
```

<span id="SIG:DATE.localOffset:VAL"></span>

### `localOffset`

```sml
val localOffset : unit -> Time.time
```
The offset from UTC for the local time zone.


```repl
Date.localOffset ();; (* local offset from UTC *)
```

<span id="SIG:DATE.fromTimeLocal:VAL"></span>

### `fromTimeLocal`

```sml
val fromTimeLocal : Time.time -> date
```

### `fromTimeUniv`

```sml
val fromTimeUniv : Time.time -> date
```
These convert the (UTC) time `t` into a corresponding date. `fromTimeLocal` represents the date in the local time zone; it is the analogue of the ISO C function `localtime`. The returned date will have `offset=`[`NONE`](option.md#SIG:OPTION.option:TY:SPEC). `fromTimeUniv` returns the date in the UTC time zone; it is the analogue of the ISO C function `gmtime`. The returned date will have `offset=`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(0)`.

If these functions are applied to the same time value, the resulting dates will differ by the offset of the local time zone from UTC.


```repl
Date.fromTimeLocal Time.zeroTime;; (* local date at the epoch *)
Date.fromTimeUniv Time.zeroTime;; (* UTC date at the epoch *)
```

<span id="SIG:DATE.toTime:VAL"></span>

### `toTime`

```sml
val toTime : date -> Time.time
```
returns the (UTC) time corresponding to the date `date`. It raises [`Date`](date.md#SIG:DATE.Date:EXN:SPEC) if the date `date` cannot be represented as a [`Time.time`](time.md#SIG:TIME.time:TY:SPEC) value. It is the analogue of the ISO C function `mktime`.


```repl
Date.toTime (Date.date {year = 1970, month = Date.Jan, day = 1, hour = 0, minute = 0, second = 0, offset = SOME Time.zeroTime});; (* 0 seconds *)
```

<span id="SIG:DATE.compare:VAL"></span>

### `compare`

```sml
val compare : date * date -> order
```
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC), according as `date1` precedes, equals, or follows `date2` in time. It lexicographically compares the dates, using the year, month, day, hour, minute, and second information, but ignoring the offset and daylight savings time information. It does not detect invalid dates.

In order to compare dates in two different time zones, the user would have to handle the normalization.


```repl
val a = Date.date {year = 2024, month = Date.Jan, day = 1, hour = 0, minute = 0, second = 0, offset = SOME Time.zeroTime};
val b = Date.date {year = 2024, month = Date.Jan, day = 2, hour = 0, minute = 0, second = 0, offset = SOME Time.zeroTime};
Date.compare (a, b);; (* LESS *)
```

<span id="SIG:DATE.fmt:VAL"></span>

### `fmt`

```sml
val fmt : string -> date -> string
```

### `toString`

```sml
val toString : date -> string
```
These return a string representation of the date `date`. The result may be wrong if the date is outside the representable [`Time.time`](time.md#SIG:TIME.time:TY:SPEC) range. They raise [`Date`](date.md#SIG:DATE.Date:EXN:SPEC) if the given date is invalid.

The former formats the date according to the format string `s`, following the semantics of the ISO C function `strftime`. In particular, [`fmt`](date.md#SIG:DATE.fmt:VAL:SPEC) is locale-dependent. The allowed formats are:

---

`%a`

locale's abbreviated weekday name

`%A`

locale's full weekday name

`%b`

locale's abbreviated month name

`%B`

locale's full month name

`%c`

locale's date and time representation (_e.g._, `"Dec 2 06:55:15 1979"`)

`%d`

day of month \[01-31\]

`%H`

hour \[00-23\]

`%I`

hour \[01-12\]

`%j`

day of year \[001-366\]

`%m`

month number \[01-12\]

`%M`

minutes \[00-59\]

`%p`

locale's equivalent of the AM/PM designation

`%S`

seconds \[00-61\]

`%U`

week number of year \[00-53\], with the first Sunday as the first day of week 01

`%w`

day of week \[0-6\], with 0 representing Sunday

`%W`

week number of year \[00-53\], with the first Monday as the first day of week 01

`%x`

locale's appropriate date representation

`%X`

locale's appropriate time representation

`%y`

year of century \[00-99\]

`%Y`

year including century (_e.g._, 1997)

`%Z`

time zone name or abbreviation, or the empty string if no time zone information exists

`%%`

the percent character

`%``c`

the character `c`, if `c` is not one of the format characters listed above

---

For instance, [`fmt`](date.md#SIG:DATE.fmt:VAL:SPEC)` "%A" ``date` returns the full name of the weekday specified by `date` (_e.g._, `"Monday"`). For a full description of the format-string syntax, consult a description of `strftime`. Note, however, that unlike `strftime`, the behavior of [`fmt`](date.md#SIG:DATE.fmt:VAL:SPEC) is defined for the directive `%``c` for any character _c_.

[`toString`](time.md#SIG:TIME.toString:VAL:SPEC) returns a 24-character string representing the date `date` in the following format:

"Wed Mar 08 19:06:45 1995"

The function is equivalent to `Date.fmt "%a %b %d %H:%M:%S %Y"`.


```repl
val d = Date.date {year = 2024, month = Date.Jan, day = 1, hour = 0, minute = 0, second = 0, offset = SOME Time.zeroTime};
Date.fmt "%Y" d;; (* "2024" *)
Date.toString d;; (* "Mon Jan 01 00:00:00 2024" *)
```

<span id="SIG:DATE.scan:VAL"></span>

### `scan`

```sml
val scan : (char, 'a) StringCvt.reader -> (date, 'a) StringCvt.reader
```

### `fromString`

```sml
val fromString : string -> date option
```
These scan a 24-character date from a character source after ignoring possible initial whitespace. The format of the string must be precisely as produced by [`toString`](date.md#SIG:DATE.toString:VAL:SPEC). In particular, the functions do not parse time zone abbreviations. No check of the consistency of the date (weekday, date in the month, ...) is performed. If the scanning fails, [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned.

The function `scan` takes a character stream reader `getc` and a stream `strm`. In case of success, it returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``date``, ``rest``)`, where `date` is the scanned date and `rest` is the remainder of the stream.

The function `fromString` takes a string `s` as its source of characters. It is equivalent to [`StringCvt.scanString`](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC)` scan`.

```repl
Date.scan Substring.getc (Substring.full "Mon Jan 01 00:00:00 2024");; (* SOME (date, empty substring) *)
Date.fromString "Mon Jan 01 00:00:00 2024";; (* SOME date *)
Date.fromString "not a date";; (* NONE *)
```


#### See Also

> [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC), [`Time`](time.md#Time:STR:SPEC)

#### Discussion

In the `Date` structure, the [`time`](time.md#SIG:TIME.time:TY:SPEC) type is used to represent intervals starting from a fixed reference point. These times are always measured in Coordinated Universal Time (UTC), also known as Greenwich Mean Time. The implementation of time values, however, is system dependent, so time values are not portable across implementations.

A conforming `Date` structure should support date values ranging from around 1900 to 2200, although they may be inaccurate with respect to daylight savings time outside the range of dates supported by time values.

> **Implementation note:**
>
> Implementations of this structure might use the ISO C `mktime` function. Some implementations of this function, when given dates that are out of range, wrap around instead of returning -1. Thus, implementations using `mktime` need to check the validity of a date before invoking the function.
