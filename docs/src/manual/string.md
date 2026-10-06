# <span id="section:0"></span>The `STRING` signature

---

#### Synopsis

<span id="STRING:SIG:SPEC"></span>
<span id="String:STR:SPEC"></span>
<span id="WideString:STR:SPEC"></span>

```sml
signature STRING
structure String :> STRING
where type string = string
where type string = CharVector.vector
where type char = Char.char
structure WideString :> STRING (* OPTIONAL *)
where type string = WideCharVector.vector
where type char = WideChar.char
```

The `STRING` signature specifies the basic operations on a string type, which is a vector of the underlying character type [`char`](char.md#SIG:CHAR.char:TY:SPEC) as defined in the structure.

The [`STRING`](string.md#STRING:SIG:SPEC) signature is matched by two structures, the required [`String`](string.md#String:STR:SPEC) and the optional [`WideString`](string.md#WideString:STR:SPEC). The former implements strings based on the extended ASCII 8-bit characters, and is a companion structure to the [`Char`](char.md#Char:STR:SPEC) structure. The latter provides strings of characters of some size greater than or equal to 8 bits, and is related to the structure [`WideChar`](char.md#WideChar:STR:SPEC). In particular, the type `String.char` is identical to the type `Char.char` and, when [`WideString`](string.md#WideString:STR:SPEC) is defined, the type `WideString.char` is identical to the type `WideChar.char`. These connections are made explicit in the [`Text`](text.md#Text:STR:SPEC) and [`WideText`](text.md#WideText:STR:SPEC) structures, which match the [`TEXT`](text.md#TEXT:SIG:SPEC) signature.

---

#### Interface

<span id="SIG:STRING.string:TY:SPEC"></span>
<span id="SIG:STRING.char:TY:SPEC"></span>
<span id="SIG:STRING.maxSize:VAL:SPEC"></span>
<span id="SIG:STRING.size:VAL:SPEC"></span>
<span id="SIG:STRING.sub:VAL:SPEC"></span>
<span id="SIG:STRING.extract:VAL:SPEC"></span>
<span id="SIG:STRING.substring:VAL:SPEC"></span>
<span id="SIG:STRING.^:VAL:SPEC"></span>
<span id="SIG:STRING.concat:VAL:SPEC"></span>
<span id="SIG:STRING.concatWith:VAL:SPEC"></span>
<span id="SIG:STRING.str:VAL:SPEC"></span>
<span id="SIG:STRING.implode:VAL:SPEC"></span>
<span id="SIG:STRING.explode:VAL:SPEC"></span>
<span id="SIG:STRING.map:VAL:SPEC"></span>
<span id="SIG:STRING.translate:VAL:SPEC"></span>
<span id="SIG:STRING.tokens:VAL:SPEC"></span>
<span id="SIG:STRING.fields:VAL:SPEC"></span>
<span id="SIG:STRING.isPrefix:VAL:SPEC"></span>
<span id="SIG:STRING.isSubstring:VAL:SPEC"></span>
<span id="SIG:STRING.isSuffix:VAL:SPEC"></span>
<span id="SIG:STRING.compare:VAL:SPEC"></span>
<span id="SIG:STRING.collate:VAL:SPEC"></span>
<span id="SIG:STRING.\|@LT\|:VAL:SPEC"></span>
<span id="SIG:STRING.\|@LTE\|:VAL:SPEC"></span>
<span id="SIG:STRING.\|@GT\|:VAL:SPEC"></span>
<span id="SIG:STRING.\|@GTE\|:VAL:SPEC"></span>
<span id="SIG:STRING.toString:VAL:SPEC"></span>
<span id="SIG:STRING.scan:VAL:SPEC"></span>
<span id="SIG:STRING.fromString:VAL:SPEC"></span>
<span id="SIG:STRING.toCString:VAL:SPEC"></span>
<span id="SIG:STRING.fromCString:VAL:SPEC"></span>

```sml
eqtype string
eqtype char
val maxSize : int
val size : string -> int
val sub : string * int -> char
val extract : string * int * int option -> string
val substring : string * int * int -> string
val ^ : string * string -> string
val concat : string list -> string
val concatWith : string -> string list -> string
val str : char -> string
val implode : char list -> string
val explode : string -> char list
val map : (char -> char) -> string -> string
val translate : (char -> string) -> string -> string
val tokens : (char -> bool) -> string -> string list
val fields : (char -> bool) -> string -> string list
val isPrefix : string -> string -> bool
val isSubstring : string -> string -> bool
val isSuffix : string -> string -> bool
val compare : string * string -> order
val collate : (char * char -> order) -> string * string -> order
val < : string * string -> bool
val <= : string * string -> bool
val > : string * string -> bool
val >= : string * string -> bool
val toString : string -> String.string
val scan : (char, 'a) StringCvt.reader -> (string, 'a) StringCvt.reader
val fromString : String.string -> string option
val toCString : string -> String.string
val fromCString : String.string -> string option
```

#### Description

<span id="SIG:STRING.maxSize:VAL"></span>

### `maxSize`

```sml
val maxSize : int
```
**`val`**` maxSize `**`:`**` int`  
The longest allowed size of a string.



```repl
String.maxSize;; (* maximum string length *)
```

<span id="SIG:STRING.size:VAL"></span>

### `size`

```sml
val size : string -> int
```

`size ``s`` `  
returns \|`s`\|, the number of characters in string `s`.



```repl
String.size "abc";; (* 3 *)
```

<span id="SIG:STRING.sub:VAL"></span>

### `sub`

```sml
val sub : string * int -> char
```

`sub (``s``, ``i``) `  
returns the `i`<sup>(th)</sup> character of `s`, counting from zero. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or \|`s`\| \<= `i`.



```repl
String.sub ("abc", 1);; (* #"b" *)
```

<span id="SIG:STRING.extract:VAL"></span>

### `extract`

```sml
val extract : string * int * int option -> string
```

### `substring`

```sml
val substring : string * int * int -> string
```

`extract (``s``, ``i``, `[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`) `
`extract (``s``, ``i``,`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``j``) `
`substring (``s``, ``i``, ``j``)`  
These return substrings of `s`. The first returns the substring of `s` from the `i`<sup>(th)</sup> character to the end of the string, _i.e._, the string `s`\[`i`..\|`s`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i`` < 0` or \|`s`\| \< `i`. The second form returns the substring of size `j` starting at index `i`, _i.e._, the string `s``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`s`\| \< `i` + `j`. Note that, if defined, [`extract`](string.md#SIG:STRING.extract:VAL:SPEC) returns the empty string when `i` = \|`s`\|.

The third form returns the substring `s``[``i``..``i``+``j``-1]`, _i.e._, the substring of size `j` starting at index `i`. This is equivalent to [`extract`](string.md#SIG:STRING.extract:VAL:SPEC)`(``s``, ``i``, `[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``j``)`.

> **Implementation note:**
>
> Implementations of these functions must perform bounds checking in such a way that the [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) exception is not raised.




```repl
String.extract ("abc", 1, NONE);; (* "bc" *)
```

```repl
String.substring ("abc", 1, 2);; (* "bc" *)
```

<span id="SIG:STRING.^:VAL"></span>

### `^`

```sml
val ^ : string * string -> string
```

`s`` ^ ``t`` `  
is the concatenation of the strings `s` and `t`. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if \|`s`\| + \|`t`\| \> [`maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).



```repl
String.^ ("ab", "cd");; (* "abcd" *)
```

<span id="SIG:STRING.concat:VAL"></span>

### `concat`

```sml
val concat : string list -> string
```

`concat ``l`` `  
is the concatenation of all the strings in `l`. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the sum of all the sizes is greater than [`maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).



```repl
String.concat ["ab", "cd"];; (* "abcd" *)
```

<span id="SIG:STRING.concatWith:VAL"></span>

### `concatWith`

```sml
val concatWith : string -> string list -> string
```

`concatWith ``s`` ``l`` `  
returns the concatenation of the strings in the list `l` using the string `s` as a separator. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the size of the resulting string would be greater than [`maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).



```repl
String.concatWith "," ["a", "b"];; (* "a,b" *)
```

<span id="SIG:STRING.str:VAL"></span>

### `str`

```sml
val str : char -> string
```

`str ``c`` `  
is the string of size one containing the character `c`.



```repl
String.str #"x";; (* "x" *)
```

<span id="SIG:STRING.implode:VAL"></span>

### `implode`

```sml
val implode : char list -> string
```

`implode ``l`` `  
generates the string containing the characters in the list `l`. This is equivalent to [`concat`](string.md#SIG:STRING.concat:VAL:SPEC)` (`[`List.map`](list.md#SIG:LIST.map:VAL:SPEC)` `[`str`](string.md#SIG:STRING.str:VAL:SPEC)` ``l``)`. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the resulting string would have size greater than [`maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC).



```repl
String.implode [#"a", #"b"];; (* "ab" *)
```

<span id="SIG:STRING.explode:VAL"></span>

### `explode`

```sml
val explode : string -> char list
```

`explode ``s`` `  
is the list of characters in the string `s`.



```repl
String.explode "ab";; (* [#"a", #"b"] *)
```

<span id="SIG:STRING.map:VAL"></span>

### `map`

```sml
val map : (char -> char) -> string -> string
```

`map ``f`` ``s`` `  
applies `f` to each element of `s` from left to right, returning the resulting string. It is equivalent to [`implode`](string.md#SIG:STRING.implode:VAL:SPEC)`(`[`List.map`](list.md#SIG:LIST.map:VAL:SPEC)` ``f`` (`[`explode`](string.md#SIG:STRING.explode:VAL:SPEC)` ``s``))`.



```repl
String.map Char.toUpper "abc";; (* "ABC" *)
```

<span id="SIG:STRING.translate:VAL"></span>

### `translate`

```sml
val translate : (char -> string) -> string -> string
```

`translate ``f`` ``s`` `  
returns the string generated from `s` by mapping each character in `s` by `f`. It is equivalent to [`concat`](string.md#SIG:STRING.concat:VAL:SPEC)`(`[`List.map`](list.md#SIG:LIST.map:VAL:SPEC)` ``f`` (`[`explode`](string.md#SIG:STRING.explode:VAL:SPEC)` ``s``))`.



```repl
String.translate (fn c => if c = #" " then "_" else String.str c) "a b";; (* "a_b" *)
```

<span id="SIG:STRING.tokens:VAL"></span>

### `tokens`

```sml
val tokens : (char -> bool) -> string -> string list
```

### `fields`

```sml
val fields : (char -> bool) -> string -> string list
```

`tokens ``f`` ``s`` `
` fields ``f`` ``s`` `  
These functions return a list of tokens or fields, respectively, derived from `s` from left to right. A token is a non-empty maximal substring of `s` not containing any delimiter. A field is a (possibly empty) maximal substring of `s` not containing any delimiter. In both cases, a delimiter is a character satisfying the predicate `f`.

Two tokens may be separated by more than one delimiter, whereas two fields are separated by exactly one delimiter. For example, if the only delimiter is the character `#"|"`, then the string `"|abc||def"` contains two tokens `"abc"` and `"def"`, whereas it contains the four fields `""`, `"abc"`, `""` and `"def"`.



```repl
String.tokens Char.isSpace " a  b ";; (* ["a", "b"] *)
```

```repl
String.fields Char.isSpace " a  b ";; (* ["", "a", "", "b", ""] *)
```

<span id="SIG:STRING.isPrefix:VAL"></span>

### `isPrefix`

```sml
val isPrefix : string -> string -> bool
```

### `isSubstring`

```sml
val isSubstring : string -> string -> bool
```

### `isSuffix`

```sml
val isSuffix : string -> string -> bool
```

`isPrefix ``s1`` ``s2`` `
` isSubstring ``s1`` ``s2`` `
` isSuffix ``s1`` ``s2`` `  
These functions return `true` if the string `s1` is a prefix, substring, or suffix (respectively) of the string `s2`. Note that the empty string is a prefix, substring, and suffix of any string, and that a string is a prefix, substring, and suffix of itself.



```repl
String.isPrefix "ab" "abc";; (* true *)
```

```repl
String.isSubstring "bc" "abc";; (* true *)
```

```repl
String.isSuffix "bc" "abc";; (* true *)
```

<span id="SIG:STRING.compare:VAL"></span>

### `compare`

```sml
val compare : string * string -> order
```

`compare (``s``, ``t``) `  
does a lexicographic comparison of the two strings using the ordering [`Char.compare`](char.md#SIG:CHAR.compare:VAL:SPEC) on the characters. It returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC), if `s` is less than, equal to, or greater than `t`, respectively.



```repl
String.compare ("a", "b");; (* LESS *)
```

<span id="SIG:STRING.collate:VAL"></span>

### `collate`

```sml
val collate : (char * char -> order) -> string * string -> order
```

`collate ``f`` (``s``, ``t``) `  
performs lexicographic comparison of the two strings using the given ordering `f` on characters.



```repl
String.collate Char.compare ("a", "b");; (* LESS *)
```

<span id="SIG:STRING.\|@LT\|:VAL"></span>

### `<`

```sml
val < : string * string -> bool
```

### `<=`

```sml
val <= : string * string -> bool
```

### `>`

```sml
val > : string * string -> bool
```

### `>=`

```sml
val >= : string * string -> bool
```
**`val`**` < `**`:`**` string `**`*`**` string `**`->`**` bool`
**`val`**` <= `**`:`**` string `**`*`**` string `**`->`**` bool`
**`val`**` > `**`:`**` string `**`*`**` string `**`->`**` bool`
**`val`**` >= `**`:`**` string `**`*`**` string `**`->`**` bool`  
These functions compare two strings lexicographically, using the underlying ordering on the [`char`](char.md#SIG:CHAR.char:TY:SPEC) type.



```repl
String.< ("a", "b");; (* true *)
```

```repl
String.<= ("a", "a");; (* true *)
```

```repl
String.> ("b", "a");; (* true *)
```

```repl
String.>= ("b", "b");; (* true *)
```

<span id="SIG:STRING.toString:VAL"></span>

### `toString`

```sml
val toString : string -> String.string
```

`toString ``s`` `  
returns a string corresponding to `s`, with non-printable characters replaced by SML escape sequences. This is equivalent to

[`translate`](string.md#SIG:STRING.translate:VAL:SPEC)` `[`Char.toString`](char.md#SIG:CHAR.toString:VAL:SPEC)` ``s`



```repl
String.toString "a\n";; (* "a\n" *)
```

<span id="SIG:STRING.scan:VAL"></span>

### `scan`

```sml
val scan : (char, 'a) StringCvt.reader -> (string, 'a) StringCvt.reader
```

### `fromString`

```sml
val fromString : String.string -> string option
```

`scan ``getc`` ``strm`` `
` fromString ``s`` `  
These functions scan their character source as a sequence of printable characters, converting SML escape sequences into the appropriate characters. They do not skip leading whitespace. They return as many characters as can successfully be scanned, stopping when they reach the end of the source or a non-printing character (_i.e._, one not satisfying [`isPrint`](char.md#SIG:CHAR.isPrint:VAL:SPEC)), or if they encounter an improper escape sequence. [`fromString`](string.md#SIG:STRING.fromString:VAL:SPEC) ignores the remaining characters, while [`scan`](string.md#SIG:STRING.scan:VAL:SPEC) returns the remaining characters as the rest of the stream.

The function [`fromString`](string.md#SIG:STRING.fromString:VAL:SPEC) is equivalent to the [`StringCvt.scanString`](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC)` scan`.

If no conversion is possible, _e.g._, if the first character is non-printable or begins an illegal escape sequence, [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. Note, however, that [`fromString`](string.md#SIG:STRING.fromString:VAL:SPEC)` ""` returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`("")`.

For more information on the allowed escape sequences, see the entry for [`CHAR.fromString`](char.md#SIG:CHAR.fromString:VAL:SPEC). SML source also allows escaped formatting sequences, which are ignored during conversion. The rule is that if any prefix of the input is successfully scanned, including an escaped formatting sequence, the functions returns some string. They only return [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) in the case where the prefix of the input cannot be scanned at all. Here are some sample conversions:

---

Input string `s`

`fromString ``s`

`"\\q"`

`NONE`

`"a\^D"`

`SOME "a"`

`"a\\ \\\\q"`

`SOME "a"`

`"\\ \\"`

`SOME ""`

`""`

`SOME ""`

`"\\ \\\^D"`

`SOME ""`

`"\\ a"`

`NONE`

---

> **Implementation note:**
>
> Because of the special cases, such as `fromString "" = SOME ""`, `fromString "\\ \\\^D" = SOME ""`, and `fromString "\^D" = NONE`, the functions cannot be implemented as a simple iterative application of [`CHAR.scan`](char.md#SIG:CHAR.scan:VAL:SPEC).




```repl
String.fromString "\"hello\"";; (* SOME "hello" *)
```

```repl
String.fromString "";; (* SOME "" *)
```

<span id="SIG:STRING.toCString:VAL"></span>

### `toCString`

```sml
val toCString : string -> String.string
```

`toCString ``s`` `  
returns a string corresponding to `s`, with non-printable characters replaced by C escape sequences. This is equivalent to

[`translate`](string.md#SIG:STRING.translate:VAL:SPEC)` `[`Char.toCString`](char.md#SIG:CHAR.toCString:VAL:SPEC)` ``s`



```repl
String.toCString "a\n";; (* "a\n" *)
```

<span id="SIG:STRING.fromCString:VAL"></span>

### `fromCString`

```sml
val fromCString : String.string -> string option
```

`fromCString ``s`` `  
scans the string `s` as a string in the C language, converting C escape sequences into the appropriate characters. The semantics are identical to [`fromString`](string.md#SIG:STRING.fromString:VAL:SPEC) above, except that C escape sequences are used (see ISO C standard ISO/IEC 9899:1990**\[CITE\]**).

For more information on the allowed escape sequences, see the entry for [`CHAR.fromCString`](char.md#SIG:CHAR.fromCString:VAL:SPEC). Note that [`fromCString`](string.md#SIG:STRING.fromCString:VAL:SPEC) accepts an unescaped single quote character, but does not accept an unescaped double quote character.



```repl
String.fromCString "a\\n";; (* SOME "a\n" *)
```

#### Examples

```repl
String.tokens Char.isSpace "";; (* [] *)
String.fields Char.isSpace "";; (* [""] *)
String.concatWith ", " ["red", "green", "blue"];; (* "red, green, blue" *)
```

#### See Also

> [`CHAR`](char.md#CHAR:SIG:SPEC), [`CharArray`](mono-array.md#CharArray:STR:SPEC), [`CharVector`](mono-vector.md#CharVector:STR:SPEC), [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC), [`SUBSTRING`](substring.md#SUBSTRING:SIG:SPEC), [`TEXT`](text.md#TEXT:SIG:SPEC), [`WideCharVector`](mono-vector.md#WideCharVector:STR:SPEC)
