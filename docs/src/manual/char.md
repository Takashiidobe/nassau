# <span id="section:0"></span>The `CHAR` signature

---

#### Synopsis

<span id="CHAR:SIG:SPEC"></span>
<span id="Char:STR:SPEC"></span>
<span id="WideChar:STR:SPEC"></span>

```sml
signature CHAR
structure Char :> CHAR
where type char = char
where type string = String.string
structure WideChar :> CHAR (* OPTIONAL *)
where type string = WideString.string
```

The `CHAR` signature defines a type `char` of characters, and provides basic operations and predicates on values of that type. There is a linear ordering defined on characters. In addition, there is an encoding of characters into a contiguous range of non-negative integers which preserves the linear ordering.

There are two structures matching the [`CHAR`](char.md#CHAR:SIG:SPEC) signature. The `Char` structure provides the extended ASCII 8-bit character set and locale-independent operations on them. For this structure, `Char.maxOrd` = 255.

The optional `WideChar` structure defines wide characters, which are represented by a fixed number of 8-bit words (bytes). If the [`WideChar`](char.md#WideChar:STR:SPEC) structure is provided, it is distinct from the [`Char`](char.md#Char:STR:SPEC) structure.

---

#### Interface

<span id="SIG:CHAR.char:TY:SPEC"></span>
<span id="SIG:CHAR.string:TY:SPEC"></span>
<span id="SIG:CHAR.minChar:VAL:SPEC"></span>
<span id="SIG:CHAR.maxChar:VAL:SPEC"></span>
<span id="SIG:CHAR.maxOrd:VAL:SPEC"></span>
<span id="SIG:CHAR.ord:VAL:SPEC"></span>
<span id="SIG:CHAR.chr:VAL:SPEC"></span>
<span id="SIG:CHAR.succ:VAL:SPEC"></span>
<span id="SIG:CHAR.pred:VAL:SPEC"></span>
<span id="SIG:CHAR.compare:VAL:SPEC"></span>
<span id="SIG:CHAR.\|@LT\|:VAL:SPEC"></span>
<span id="SIG:CHAR.\|@LTE\|:VAL:SPEC"></span>
<span id="SIG:CHAR.\|@GT\|:VAL:SPEC"></span>
<span id="SIG:CHAR.\|@GTE\|:VAL:SPEC"></span>
<span id="SIG:CHAR.contains:VAL:SPEC"></span>
<span id="SIG:CHAR.notContains:VAL:SPEC"></span>
<span id="SIG:CHAR.isAscii:VAL:SPEC"></span>
<span id="SIG:CHAR.toLower:VAL:SPEC"></span>
<span id="SIG:CHAR.toUpper:VAL:SPEC"></span>
<span id="SIG:CHAR.isAlpha:VAL:SPEC"></span>
<span id="SIG:CHAR.isAlphaNum:VAL:SPEC"></span>
<span id="SIG:CHAR.isCntrl:VAL:SPEC"></span>
<span id="SIG:CHAR.isDigit:VAL:SPEC"></span>
<span id="SIG:CHAR.isGraph:VAL:SPEC"></span>
<span id="SIG:CHAR.isHexDigit:VAL:SPEC"></span>
<span id="SIG:CHAR.isLower:VAL:SPEC"></span>
<span id="SIG:CHAR.isPrint:VAL:SPEC"></span>
<span id="SIG:CHAR.isSpace:VAL:SPEC"></span>
<span id="SIG:CHAR.isPunct:VAL:SPEC"></span>
<span id="SIG:CHAR.isUpper:VAL:SPEC"></span>
<span id="SIG:CHAR.toString:VAL:SPEC"></span>
<span id="SIG:CHAR.scan:VAL:SPEC"></span>
<span id="SIG:CHAR.fromString:VAL:SPEC"></span>
<span id="SIG:CHAR.toCString:VAL:SPEC"></span>
<span id="SIG:CHAR.fromCString:VAL:SPEC"></span>

```sml
eqtype char
eqtype string
val minChar : char
val maxChar : char
val maxOrd : int
val ord : char -> int
val chr : int -> char
val succ : char -> char
val pred : char -> char
val compare : char * char -> order
val < : char * char -> bool
val <= : char * char -> bool
val > : char * char -> bool
val >= : char * char -> bool
val contains : string -> char -> bool
val notContains : string -> char -> bool
val isAscii : char -> bool
val toLower : char -> char
val toUpper : char -> char
val isAlpha : char -> bool
val isAlphaNum : char -> bool
val isCntrl : char -> bool
val isDigit : char -> bool
val isGraph : char -> bool
val isHexDigit : char -> bool
val isLower : char -> bool
val isPrint : char -> bool
val isSpace : char -> bool
val isPunct : char -> bool
val isUpper : char -> bool
val toString : char -> String.string
val scan : (Char.char, 'a) StringCvt.reader -> (char, 'a) StringCvt.reader
val fromString : String.string -> char option
val toCString : char -> String.string
val fromCString : String.string -> char option
```

#### Description

<span id="SIG:CHAR.minChar:VAL"></span>

### `minChar`

```sml
val minChar : char
```
The least character in the ordering. It always equals [`chr`](char.md#SIG:CHAR.chr:VAL:SPEC)` 0`.


```repl
Char.minChar;; (* #"\000" *)
```

<span id="SIG:CHAR.maxChar:VAL"></span>

### `maxChar`

```sml
val maxChar : char
```
The greatest character in the ordering; it equals [`chr`](char.md#SIG:CHAR.chr:VAL:SPEC)` `[`maxOrd`](char.md#SIG:CHAR.maxOrd:VAL:SPEC).


```repl
Char.maxChar;; (* #"\255" *)
```

<span id="SIG:CHAR.maxOrd:VAL"></span>

### `maxOrd`

```sml
val maxOrd : int
```
The greatest character code; it equals [`ord`](char.md#SIG:CHAR.ord:VAL:SPEC)` `[`maxChar`](char.md#SIG:CHAR.maxChar:VAL:SPEC).


```repl
Char.maxOrd;; (* 255 *)
```

<span id="SIG:CHAR.ord:VAL"></span>

### `ord`

```sml
val ord : char -> int
```
returns the (non-negative) integer code of the character `c`.


```repl
Char.ord #"A";; (* 65 *)
```

<span id="SIG:CHAR.chr:VAL"></span>

### `chr`

```sml
val chr : int -> char
```
returns the character whose code is `i`; raises [`Chr`](general.md#SIG:GENERAL.Chr:EXN:SPEC) if `i` \< 0 or `i` \> `maxOrd`.


```repl
Char.chr 65;; (* #"A" *)
```

<span id="SIG:CHAR.succ:VAL"></span>

### `succ`

```sml
val succ : char -> char
```
returns the character immediately following `c` in the ordering, or raises [`Chr`](general.md#SIG:GENERAL.Chr:EXN:SPEC) if `c` = `maxChar`. When defined, [`succ`](char.md#SIG:CHAR.succ:VAL:SPEC)` ``c` is equivalent to [`chr`](char.md#SIG:CHAR.chr:VAL:SPEC)`(`[`ord`](char.md#SIG:CHAR.ord:VAL:SPEC)` ``c`` + 1)`.


```repl
Char.succ #"A";; (* #"B" *)
```

<span id="SIG:CHAR.pred:VAL"></span>

### `pred`

```sml
val pred : char -> char
```
returns the character immediately preceding `c`, or raises [`Chr`](general.md#SIG:GENERAL.Chr:EXN:SPEC) if `c` = `minChar`. When defined, [`pred`](char.md#SIG:CHAR.pred:VAL:SPEC)` ``c` is equivalent to [`chr`](char.md#SIG:CHAR.chr:VAL:SPEC)`(`[`ord`](char.md#SIG:CHAR.ord:VAL:SPEC)` ``c`` - 1)`.


```repl
Char.pred #"B";; (* #"A" *)
```

<span id="SIG:CHAR.compare:VAL"></span>

### `compare`

```sml
val compare : char * char -> order
```
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC), depending on whether `c` precedes, equals, or follows `d` in the character ordering.


```repl
Char.compare (#"a", #"b");; (* LESS *)
```

<span id="SIG:CHAR.\|@LT\|:VAL"></span>

### `<`

```sml
val < : char * char -> bool
```

### `<=`

```sml
val <= : char * char -> bool
```

### `>`

```sml
val > : char * char -> bool
```

### `>=`

```sml
val >= : char * char -> bool
```
These compare characters in the character ordering. Note that the functions [`ord`](char.md#SIG:CHAR.ord:VAL:SPEC) and [`chr`](char.md#SIG:CHAR.chr:VAL:SPEC) preserve orderings. For example, if we have `x`` < ``y` for characters `x` and `y`, then it is also true that `ord ``x`` < ord ``y`.


```repl
Char.< (#"a", #"b");; (* true *)
Char.<= (#"a", #"a");; (* true *)
Char.> (#"b", #"a");; (* true *)
Char.>= (#"b", #"b");; (* true *)
```

<span id="SIG:CHAR.contains:VAL"></span>

### `contains`

```sml
val contains : string -> char -> bool
```
returns `true` if character `c` occurs in the string `s`; otherwise it returns `false`.

> **Implementation note:**
>
> In some implementations, the partial application of [`contains`](char.md#SIG:CHAR.contains:VAL:SPEC) to `s` may build a table, which is used by the resulting function to decide whether a given character is in the string or not. Hence `val ``p`` = `[`contains`](char.md#SIG:CHAR.contains:VAL:SPEC)` ``s` may be expensive to compute, but `p ``c` might be fast for any given character `c`.



```repl
Char.contains "Nassau" #"s";; (* true *)
```

<span id="SIG:CHAR.notContains:VAL"></span>

### `notContains`

```sml
val notContains : string -> char -> bool
```
returns `true` if character `c` does not occur in the string `s`; it returns `false` otherwise. It is equivalent to [`not`](bool.md#SIG:BOOL.not:VAL:SPEC)`(`[`contains`](char.md#SIG:CHAR.contains:VAL:SPEC)` ``s`` ``c`).

> **Implementation note:**
>
> As with [`contains`](char.md#SIG:CHAR.contains:VAL:SPEC), [`notContains`](char.md#SIG:CHAR.notContains:VAL:SPEC) may be implemented via table lookup.



```repl
Char.notContains "Nassau" #"x";; (* true *)
```

<span id="SIG:CHAR.isAscii:VAL"></span>

### `isAscii`

```sml
val isAscii : char -> bool
```
returns `true` if `c` is a (seven-bit) ASCII character, _i.e._, 0 \<= `ord` `c` \<= 127. Note that this function is independent of locale.


```repl
Char.isAscii #"A";; (* true *)
```

<span id="SIG:CHAR.toLower:VAL"></span>

### `toLower`

```sml
val toLower : char -> char
```

### `toUpper`

```sml
val toUpper : char -> char
```
These return the lowercase (respectively, uppercase) letter corresponding to `c` if `c` is a letter; otherwise it returns `c`.


```repl
Char.toLower #"A";; (* #"a" *)
Char.toUpper #"b";; (* #"B" *)
```

<span id="SIG:CHAR.isAlpha:VAL"></span>

### `isAlpha`

```sml
val isAlpha : char -> bool
```
returns `true` if `c` is a letter (lowercase or uppercase).


```repl
Char.isAlpha #"A";; (* true *)
```

<span id="SIG:CHAR.isAlphaNum:VAL"></span>

### `isAlphaNum`

```sml
val isAlphaNum : char -> bool
```
returns `true` if `c` is alphanumeric (a letter or a decimal digit).


```repl
Char.isAlphaNum #"7";; (* true *)
```

<span id="SIG:CHAR.isCntrl:VAL"></span>

### `isCntrl`

```sml
val isCntrl : char -> bool
```
returns `true` if `c` is a control character.


```repl
Char.isCntrl #"\n";; (* true *)
```

<span id="SIG:CHAR.isDigit:VAL"></span>

### `isDigit`

```sml
val isDigit : char -> bool
```
returns `true` if `c` is a decimal digit \[`0`-`9`\].


```repl
Char.isDigit #"7";; (* true *)
```

<span id="SIG:CHAR.isGraph:VAL"></span>

### `isGraph`

```sml
val isGraph : char -> bool
```
returns `true` if `c` is a graphical character, that is, it is printable and not a whitespace character.


```repl
Char.isGraph #"!";; (* true *)
```

<span id="SIG:CHAR.isHexDigit:VAL"></span>

### `isHexDigit`

```sml
val isHexDigit : char -> bool
```
returns `true` if `c` is a hexadecimal digit \[`0`-`9``a`-`f``A`-`F`\].


```repl
Char.isHexDigit #"F";; (* true *)
```

<span id="SIG:CHAR.isLower:VAL"></span>

### `isLower`

```sml
val isLower : char -> bool
```
returns `true` if `c` is a lowercase letter.


```repl
Char.isLower #"a";; (* true *)
```

<span id="SIG:CHAR.isPrint:VAL"></span>

### `isPrint`

```sml
val isPrint : char -> bool
```
returns `true` if `c` is a printable character (space or visible), _i.e._, not a control character.


```repl
Char.isPrint #" ";; (* true *)
```

<span id="SIG:CHAR.isSpace:VAL"></span>

### `isSpace`

```sml
val isSpace : char -> bool
```
returns `true` if `c` is a whitespace character (space, newline, tab, carriage return, vertical tab, formfeed).


```repl
Char.isSpace #"\t";; (* true *)
```

<span id="SIG:CHAR.isPunct:VAL"></span>

### `isPunct`

```sml
val isPunct : char -> bool
```
returns `true` if `c` is a punctuation character: graphical but not alphanumeric.


```repl
Char.isPunct #"!";; (* true *)
```

<span id="SIG:CHAR.isUpper:VAL"></span>

### `isUpper`

```sml
val isUpper : char -> bool
```
returns `true` if `c` is an uppercase letter.


```repl
Char.isUpper #"A";; (* true *)
```

<span id="SIG:CHAR.toString:VAL"></span>

### `toString`

```sml
val toString : char -> String.string
```
returns a printable string representation of the character, using, if necessary, SML escape sequences. Printable characters, except for `#"\\"` and `#"\""`, are left unchanged. Backslash `#"\\"` becomes `"\\\\"`; double quote `#"\""` becomes `"\\\""`. The common control characters are converted to two-character escape sequences:

---

Alert (ASCII 0x07)

`"\\a"`

Backspace (ASCII 0x08)

`"\\b"`

Horizontal tab (ASCII 0x09)

`"\\t"`

Linefeed or newline (ASCII 0x0A)

`"\\n"`

Vertical tab (ASCII 0x0B)

`"\\v"`

Form feed (ASCII 0x0C)

`"\\f"`

Carriage return (ASCII 0x0D)

`"\\r"`

---

The remaining characters whose codes are less than 32 are represented by three-character strings in \`\`control character'' notation, _e.g._, `#"\000"` maps to `"\\^@"`, `#"\001"` maps to `"\\^A"`, etc. For characters whose codes are greater than 999, the character is mapped to a six-character string of the form `"\\uxxxx"`, where `xxxx` are the four hexadecimal digits corresponding to a character's code. All other characters (_i.e._, those whose codes are greater than 126 but less than 1000) are mapped to four-character strings of the form `"\\ddd"`, where `ddd` are the three decimal digits corresponding to a character's code.

To convert a character to a length-one string containing the character, use the function [`String.str`](string.md#SIG:STRING.str:VAL:SPEC).


```repl
Char.toString #"\n";; (* "\n" *)
```

<span id="SIG:CHAR.scan:VAL"></span>

### `scan`

```sml
val scan : (Char.char, 'a) StringCvt.reader -> (char, 'a) StringCvt.reader
```

### `fromString`

```sml
val fromString : String.string -> char option
```
These scan a character (including possibly a space) or an SML escape sequence representing a character from the prefix of a character stream or a string of printable characters, as allowed in an SML program. After a successful conversion, [`scan`](char.md#SIG:CHAR.scan:VAL:SPEC) returns the remainder of the stream along with the character, whereas [`fromString`](char.md#SIG:CHAR.fromString:VAL:SPEC) ignores any additional characters in `s` and just returns the character. If the first character is non-printable (_i.e._, not in the ASCII range \[0x20,0x7E\]) or starts an illegal escape sequence (_e.g._, `"\q"`), no conversion is possible and [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. The function `fromString` is equivalent to [`StringCvt.scanString`](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC)` scan`.

The allowable escape sequences are:

---

`\a`

Alert (ASCII 0x07)

`\b`

Backspace (ASCII 0x08)

`\t`

Horizontal tab (ASCII 0x09)

`\n`

Linefeed or newline (ASCII 0x0A)

`\v`

Vertical tab (ASCII 0x0B)

`\f`

Form feed (ASCII 0x0C)

`\r`

Carriage return (ASCII 0x0D)

`\\`

Backslash

`\"`

Double quote

`\^``c`

A control character whose encoding is `ord ``c`` - 64`, with the

character `c` having `ord ``c` in the range \[64,95\]. For example,

`\^H` (control-H) is the same as `\b` (backspace).

`\``ddd`

The character whose encoding is the number `ddd`, three decimal

digits denoting an integer in the range \[0,255\].

`\u``xxxx`

The character whose encoding is the number `xxxx`, four hexadecimal

digits denoting an integer in the ordinal range of the alphabet.

`\``f...f``\`

This sequence is ignored, where `f...f` stands for a sequence of one

or more formatting (space, newline, tab, etc.) characters.

---

In the escape sequences involving decimal or hexadecimal digits, if the resulting value cannot be represented in the character set, [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. As the table indicates, escaped formatting sequences (`\f...f\`) are passed over during scanning. Such sequences are successfully scanned, so that the remaining stream returned by [`scan`](char.md#SIG:CHAR.scan:VAL:SPEC) will never have a valid escaped formatting sequence as its prefix.

Here are some sample conversions:

---

Input string `s`

`fromString ``s`

`"\\q"`

`NONE`

`"a\^D"`

`SOME #"a"`

`"a\\ \\\q"`

`SOME #"a"`

`"\\ \\"`

`NONE`

`""`

`NONE`

`"\\ \\\^D"`

`NONE`

`"\\ a"`

`NONE`

---



```repl
Char.scan Substring.getc (Substring.full "A");; (* SOME (#"A", empty substring) *)
Char.fromString "A";; (* SOME #"A" *)
Char.fromString "";; (* NONE *)
```

<span id="SIG:CHAR.toCString:VAL"></span>

### `toCString`

```sml
val toCString : char -> String.string
```
returns a printable string corresponding to `c`, with non-printable characters replaced by C escape sequences. Specifically, printable characters, except for `#"\\"`, `#"\""`, `#"?"`, and `#"'"` are left unchanged. Backslash (`#"\\"`) becomes `"\\\\"`; double quote (`#"\""`) becomes `"\\\""`, question mark (`#"?"`) becomes `"\\?"`, and single quote (`#"'"`) becomes `"\\'"`. The common control characters are converted to two-character escape sequences:

---

Alert (ASCII 0x07)

`"\\a"`

Backspace (ASCII 0x08)

`"\\b"`

Horizontal tab (ASCII 0x09)

`"\\t"`

Linefeed or newline (ASCII 0x0A)

`"\\n"`

Vertical tab (ASCII 0x0B)

`"\\v"`

Form feed (ASCII 0x0C)

`"\\f"`

Carriage return (ASCII 0x0D)

`"\\r"`

---

All other characters are represented by three octal digits, corresponding to a character's code, preceded by a backslash.


```repl
Char.toCString #"\n";; (* "\n" *)
```

<span id="SIG:CHAR.fromCString:VAL"></span>

### `fromCString`

```sml
val fromCString : String.string -> char option
```
scans a character (including possibly a space) or a C escape sequence representing a character from the prefix of a string. After a successful conversion, [`fromCString`](char.md#SIG:CHAR.fromCString:VAL:SPEC) ignores any additional characters in `s`. If no conversion is possible, _e.g._, if the first character is non-printable (_i.e._, not in the ASCII range \[0x20-0x7E\] or starts an illegal escape sequence, [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned.

The allowable escape sequences are given below (_cf_. Section 6.1.3.4 of the ISO C standard ISO/IEC 9899:1990**\[CITE\]**).

---

`\a`

Alert (ASCII 0x07)

`\b`

Backspace (ASCII 0x08)

`\t`

Horizontal tab (ASCII 0x09)

`\n`

Linefeed or newline (ASCII 0x0A)

`\v`

Vertical tab (ASCII 0x0B)

`\f`

Form feed (ASCII 0x0C)

`\r`

Carriage return (ASCII 0x0D)

`\?`

Question mark

`\\`

Backslash

`\"`

Double quote

`\'`

Single quote

`\^``c`

A control character whose encoding is `ord ``c`` - 64`, with the

character `c` having `ord ``c` in the range \[64,95\]. For example,

`\^H` (control-H) is the same as `\b` (backspace).

`\``ooo`

The character whose encoding is the number `ooo`, where

`ooo` consists of one to three octal digits

`\x``hh`

The character whose encoding is the number `hh`,

where `hh` is a sequence of hexadecimal digits.

---

Note that [`fromCString`](char.md#SIG:CHAR.fromCString:VAL:SPEC) accepts an unescaped single quote character, but does not accept an unescaped double quote character.

In the escape sequences involving octal or hexadecimal digits, the sequence of digits is taken to be the longest sequence of such characters. If the resulting value cannot be represented in the character set, [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned.

```repl
Char.fromCString "\n";; (* SOME #"\n" *)
Char.fromCString "";; (* NONE *)
```


#### See Also

> [`STRING`](string.md#STRING:SIG:SPEC), [`TEXT`](text.md#TEXT:SIG:SPEC)

#### Discussion

In [`WideChar`](char.md#WideChar:STR:SPEC), the functions [`toLower`](char.md#SIG:CHAR.toLower:VAL:SPEC), [`toLower`](char.md#SIG:CHAR.toLower:VAL:SPEC), [`isAlpha`](char.md#SIG:CHAR.isAlpha:VAL:SPEC),..., [`isUpper`](char.md#SIG:CHAR.isUpper:VAL:SPEC) and, in general, the definition of a \`\`letter'' are locale-dependent. In [`Char`](char.md#Char:STR:SPEC), these functions are locale-independent, with the following semantics:

---

`isUpper` `c`

`#"A" <= ``c`` `**`andalso`**` ``c`` <= #"Z"`

`isLower` `c`

`#"a" <= ``c`` `**`andalso`**` ``c`` <= #"z"`

`isDigit` `c`

`#"0" <= ``c`` `**`andalso`**` ``c`` <= #"9"`

`isAlpha` `c`

`isUpper ``c`` `**`orelse`**` isLower ``c`

`isAlphaNum` `c`

`isAlpha ``c`` `**`orelse`**` isDigit ``c`

`isHexDigit` `c`

`isDigit ``c`

**`orelse`**` (#"a" <= ``c`` `**`andalso`**` ``c`` <= #"f")`

**`orelse`**` (#"A" <= ``c`` `**`andalso`**` ``c`` <= #"F")`

`isGraph` `c`

`#"!" <= ``c`` `**`andalso`**` ``c`` <= #"~"`

`isPrint` `c`

`isGraph ``c`` `**`orelse`**` ``c`` = #" "`

`isPunct` `c`

`isGraph ``c`` `**`andalso`**` not (isAlphaNum ``c``)`

`isCntrl` `c`

`isAscii ``c`` `**`andalso`**` not (isPrint ``c``)`

`isSpace` `c`

`(#"\t" <= ``c`` `**`andalso`**` ``c`` <= #"\r")`

**`orelse`**` ``c`` = #" "`

`isAscii` `c`

`0 <= ord ``c`` `**`andalso`**` ord ``c`` <= 127`

`toLower` `c`

**`if`**` isUpper ``c`` `**`then`**`chr (ord ``c`` + 32)`**`else`**` ``c`

`toUpper` `c`

**`if`**` isLower ``c`` `**`then`**`chr (ord ``c`` - 32)`**`else`**` ``c`

---
