# <span id="section:0"></span>The `SUBSTRING` signature

---

#### Synopsis

<span id="SUBSTRING:SIG:SPEC"></span>
<span id="Substring:STR:SPEC"></span>
<span id="WideSubstring:STR:SPEC"></span>

```sml
signature SUBSTRING
structure Substring :> SUBSTRING
where type substring = CharVectorSlice.slice
where type string = String.string
where type char = Char.char
structure WideSubstring :> SUBSTRING (* OPTIONAL *)
where type substring = WideCharVectorSlice.slice
where type string = WideString.string
where type char = WideChar.char
```

The `SUBSTRING` signature specifies manipulations on an abstract representation of a sequence of contiguous characters in a string. A `substring` value can be modeled as a triple `(``s``, ``i``, ``n``)`, where `s` is the underlying string, `i` is the starting index, and `n` is the size of the substring, with the constraint that 0 \<= `i` \<= `i` + `n` \<= \|`s`\|.

The `substring` type and its attendant functions provide a convenient abstraction for performing a variety of common analyses of strings, such as finding the leftmost occurrence, if any, of a character in a string. In addition, using the `substring` functions avoids much of the copying and bounds checking that occur if similar operations are implemented solely in terms of strings.

The `SUBSTRING` signature is matched by two structures, the required `Substring` and the optional `WideSubstring`. The former is a companion structure to the [`Char`](char.md#Char:STR:SPEC) and [`String`](string.md#String:STR:SPEC) structures, which are based on the extended ASCII 8-bit character set. The structure [`WideSubstring`](substring.md#WideSubstring:STR:SPEC) is related in the same way to the structures [`WideChar`](char.md#WideChar:STR:SPEC) and [`WideString`](string.md#WideString:STR:SPEC), which are based on characters of some size greater than or equal to 8 bits. In particular, the types `Substring.string` and `Substring.char` are identical to those types in the structure [`String`](string.md#String:STR:SPEC) and, when [`WideSubstring`](substring.md#WideSubstring:STR:SPEC) is defined, the types `WideSubstring.string` and `WideSubstring.char` are identical to those types in the structure [`WideString`](string.md#WideString:STR:SPEC).

All of these connections are made explicit in the [`Text`](text.md#Text:STR:SPEC) and [`WideText`](text.md#WideText:STR:SPEC) structures, which match the [`TEXT`](text.md#TEXT:SIG:SPEC) signature. In the exposition below, references to a `String` structure refers to the substructure of that name defined in either the [`Text`](text.md#Text:STR:SPEC) or the [`WideText`](text.md#WideText:STR:SPEC) structure, which ever is appropriate.

The design of the [`SUBSTRING`](substring.md#SUBSTRING:SIG:SPEC) interface was influenced by the paper \`\`Subsequence References: First-Class Values for Substrings,'' by Wilfred J. Hansen**\[CITE\]**.

---

#### Interface

<span id="SIG:SUBSTRING.substring:TY:SPEC"></span>
<span id="SIG:SUBSTRING.char:TY:SPEC"></span>
<span id="SIG:SUBSTRING.string:TY:SPEC"></span>
<span id="SIG:SUBSTRING.sub:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.size:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.base:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.extract:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.substring:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.full:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.string:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.isEmpty:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.getc:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.first:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.triml:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.trimr:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.slice:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.concat:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.concatWith:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.explode:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.isPrefix:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.isSubstring:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.isSuffix:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.compare:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.collate:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.splitl:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.splitr:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.splitAt:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.dropl:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.dropr:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.takel:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.taker:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.position:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.span:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.translate:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.tokens:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.fields:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.app:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.foldl:VAL:SPEC"></span>
<span id="SIG:SUBSTRING.foldr:VAL:SPEC"></span>

```sml
type substring
eqtype char
eqtype string
val sub : substring * int -> char
val size : substring -> int
val base : substring -> string * int * int
val extract : string * int * int option -> substring
val substring : string * int * int -> substring
val full : string -> substring
val string : substring -> string
val isEmpty : substring -> bool
val getc : substring -> (char * substring) option
val first : substring -> char option
val triml : int -> substring -> substring
val trimr : int -> substring -> substring
val slice : substring * int * int option -> substring
val concat : substring list -> string
val concatWith : string -> substring list -> string
val explode : substring -> char list
val isPrefix : string -> substring -> bool
val isSubstring : string -> substring -> bool
val isSuffix : string -> substring -> bool
val compare : substring * substring -> order
val collate : (char * char -> order) -> substring * substring -> order
val splitl : (char -> bool) -> substring -> substring * substring
val splitr : (char -> bool) -> substring -> substring * substring
val splitAt : substring * int -> substring * substring
val dropl : (char -> bool) -> substring -> substring
val dropr : (char -> bool) -> substring -> substring
val takel : (char -> bool) -> substring -> substring
val taker : (char -> bool) -> substring -> substring
val position : string -> substring -> substring * substring
val span : substring * substring -> substring
val translate : (char -> string) -> substring -> string
val tokens : (char -> bool) -> substring -> substring list
val fields : (char -> bool) -> substring -> substring list
val app : (char -> unit) -> substring -> unit
val foldl : (char * 'a -> 'a) -> 'a -> substring -> 'a
val foldr : (char * 'a -> 'a) -> 'a -> substring -> 'a
```

#### Description

<span id="SIG:SUBSTRING.sub:VAL"></span>

### `sub`

```sml
val sub : substring * int -> char
```

`sub (``s``, ``i``) `  
returns the `i`<sup>(th)</sup> character in the substring, counting from the beginning of `s`. It is equivalent to [`String.sub`](string.md#SIG:STRING.sub:VAL:SPEC)`(`[`string`](substring.md#SIG:SUBSTRING.string:VAL:SPEC)` ``s``, ``i``)`. The exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised unless 0 \<= `i` \< \|`s`\|.



```repl
Substring.sub (Substring.full "abc", 1);; (* #"b" *)
```

<span id="SIG:SUBSTRING.size:VAL"></span>

### `size`

```sml
val size : substring -> int
```

`size ``s`` `  
returns the size of `s`. This is equivalent to `#3 o `[`base`](substring.md#SIG:SUBSTRING.base:VAL:SPEC) and [`String.size`](string.md#SIG:STRING.size:VAL:SPEC)`o`[`string`](substring.md#SIG:SUBSTRING.string:VAL:SPEC).



```repl
Substring.size (Substring.full "abc");; (* 3 *)
```

<span id="SIG:SUBSTRING.base:VAL"></span>

### `base`

```sml
val base : substring -> string * int * int
```

`base ``ss`` `  
returns a triple `(``s``, ``i``, ``n``)` giving a concrete representation of the substring. `s` is the underlying string, `i` is the starting index, and `n` is the size of the substring. It will always be the case that 0 \<= `i` \<= `i` + `n` \<= \|`s`\| .



```repl
Substring.base (Substring.full "abc");; (* ("abc", 0, 3) *)
```

<span id="SIG:SUBSTRING.extract:VAL"></span>

### `extract`

```sml
val extract : string * int * int option -> substring
```

### `substring`

```sml
val substring : string * int * int -> substring
```

`extract (``s``, ``i``, `[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`) `
`extract (``s``, ``i``,`[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``j``) `
`substring (``s``, ``i``, ``j``)`  
The first returns the substring of `s` from the `i`<sup>(th)</sup> character to the end of the string, _i.e._, the string `s`\[`i`..\|`s`\|-1\]. This raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) unless 0 \<= `i` \<= \|`s`\|. The second form returns the substring of size `j` starting at index `i`, _i.e._, the string `s``[``i``..``i``+``j``-1]`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `j` \< 0 or \|`s`\| \< `i` + `j`. Note that, if defined, [`extract`](substring.md#SIG:SUBSTRING.extract:VAL:SPEC) returns the empty substring when `i` = \|`s`\|.

The third form returns the substring `s``[``i``..``i``+``j``-1]`, _i.e._, the substring of size `j` starting at index `i`. This is equivalent to [`extract`](string.md#SIG:STRING.extract:VAL:SPEC)`(``s``, ``i``, `[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``j``)`.

We require that [`base`](substring.md#SIG:SUBSTRING.base:VAL:SPEC)`o`[`substring`](substring.md#SIG:SUBSTRING.substring:VAL:SPEC) be the identity function on valid arguments.

> **Implementation note:**
>
> Implementations of these functions must perform bounds checking in such a way that the [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) exception is not raised.




```repl
Substring.string (Substring.extract ("abc", 1, NONE));; (* "bc" *)
```

```repl
Substring.string (Substring.substring ("abc", 1, 2));; (* "bc" *)
```

<span id="SIG:SUBSTRING.full:VAL"></span>

### `full`

```sml
val full : string -> substring
```

`full ``s`` `  
creates a substring representing the entire string `s`. It is equivalent to the expression [`substring`](substring.md#SIG:SUBSTRING.substring:VAL:SPEC)`(``s``, 0, `[`String.size`](string.md#SIG:STRING.size:VAL:SPEC)` ``s``)`.



```repl
Substring.string (Substring.full "abc");; (* "abc" *)
```

<span id="SIG:SUBSTRING.string:VAL"></span>

### `string`

```sml
val string : substring -> string
```

`string ``s`` `  
creates a string value corresponding to the substring. It is equivalent to [`String.substring`](string.md#SIG:STRING.substring:VAL:SPEC)`o`[`base`](substring.md#SIG:SUBSTRING.base:VAL:SPEC) for the corresponding `String` structure.



```repl
Substring.string (Substring.full "abc");; (* "abc" *)
```

<span id="SIG:SUBSTRING.isEmpty:VAL"></span>

### `isEmpty`

```sml
val isEmpty : substring -> bool
```

`isEmpty ``s`` `  
returns `true` if `s` has size 0.



```repl
Substring.isEmpty (Substring.full "");; (* true *)
```

<span id="SIG:SUBSTRING.getc:VAL"></span>

### `getc`

```sml
val getc : substring -> (char * substring) option
```

`getc ``s`` `  
returns the first character in `s` and the rest of the substring, or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `s` is empty.



```repl
Option.isSome (Substring.getc (Substring.full "abc"));; (* true *)
```

<span id="SIG:SUBSTRING.first:VAL"></span>

### `first`

```sml
val first : substring -> char option
```

`first ``s`` `  
returns the first character in `s`, or [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if `s` is empty.



```repl
Substring.first (Substring.full "abc");; (* SOME #"a" *)
```

<span id="SIG:SUBSTRING.triml:VAL"></span>

### `triml`

```sml
val triml : int -> substring -> substring
```

### `trimr`

```sml
val trimr : int -> substring -> substring
```

`triml ``k`` ``s`` `
` trimr ``k`` ``s`` `  
These functions remove `k` characters from the left (respectively, right) of the substring `s`. If `k` is greater than the size of the substring, an empty substring is returned. Specifically, for substring `ss`` = `[`substring`](substring.md#SIG:SUBSTRING.substring:VAL:SPEC)`(``s``, ``i``, ``j``)` and `k` \<= `j`, we have:

[triml](substring.md#SIG:SUBSTRING.triml:VAL:SPEC) `k` `ss` = [substring](substring.md#SIG:SUBSTRING.substring:VAL:SPEC)(`s`, `i`+`k`, `j`-`k`)
[trimr](substring.md#SIG:SUBSTRING.trimr:VAL:SPEC) `k` `ss` = [substring](substring.md#SIG:SUBSTRING.substring:VAL:SPEC)(`s`, `i`, `j`-`k`)

The exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised if `k`` < 0`. This exception is raised when `triml ``k` or `trimr ``k` is evaluated.



```repl
Substring.string (Substring.triml 1 (Substring.full "abc"));; (* "bc" *)
```

```repl
Substring.string (Substring.trimr 1 (Substring.full "abc"));; (* "ab" *)
```

<span id="SIG:SUBSTRING.slice:VAL"></span>

### `slice`

```sml
val slice : substring * int * int option -> substring
```

`slice (``s``, ``i``, `[`SOME`](option.md#SIG:OPTION.option:TY:SPEC)` ``m``) `
`slice (``s``, ``i``,`[`NONE`](option.md#SIG:OPTION.option:TY:SPEC)`) `  
These return a substring of `s` starting at the `i`<sup>(th)</sup> character. In the former case, the size of the resulting substring is `m`. Otherwise, the size is \|`s`\| - `i`. To be valid, the arguments in the first case must satisfy 0 \<= `i`, 0 \<= `m` and `i` + `m` \<= \|`s`\|. In the second case, the arguments must satisfy 0 \<= `i` \<= \|`s`\|. If the arguments are not valid, the exception [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) is raised.



```repl
Substring.string (Substring.slice ("abc", 1, SOME 2));; (* "bc" *)
```

<span id="SIG:SUBSTRING.concat:VAL"></span>

### `concat`

```sml
val concat : substring list -> string
```

`concat ``l`` `  
generates a string that is the concatenation of the substrings in `l`. This is equivalent to [`String.concat`](string.md#SIG:STRING.concat:VAL:SPEC)` o (`[`List.map`](list.md#SIG:LIST.map:VAL:SPEC)` `[`string`](substring.md#SIG:SUBSTRING.string:VAL:SPEC)`)`. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the sum of all the sizes is greater than the corresponding [`maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC) for the [`string`](substring.md#SIG:SUBSTRING.string:TY:SPEC) type.



```repl
Substring.concat [Substring.full "ab", Substring.full "cd"];; (* "abcd" *)
```

<span id="SIG:SUBSTRING.concatWith:VAL"></span>

### `concatWith`

```sml
val concatWith : string -> substring list -> string
```

`concatWith ``s`` ``l`` `  
returns the concatenation of the substrings in the list `l` using the string `s` as a separator. This raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the size of the resulting string would be greater than [`maxSize`](string.md#SIG:STRING.maxSize:VAL:SPEC) for the [`string`](substring.md#SIG:SUBSTRING.string:TY:SPEC) type.



```repl
Substring.concatWith "," [Substring.full "a", Substring.full "b"];; (* "a,b" *)
```

<span id="SIG:SUBSTRING.explode:VAL"></span>

### `explode`

```sml
val explode : substring -> char list
```

`explode ``s`` `  
returns the list of characters composing the substring. This is equivalent to [`String.explode`](string.md#SIG:STRING.explode:VAL:SPEC)` (`[`string`](substring.md#SIG:SUBSTRING.string:VAL:SPEC)` ``s``)`.



```repl
Substring.explode (Substring.full "ab");; (* [#"a", #"b"] *)
```

<span id="SIG:SUBSTRING.isPrefix:VAL"></span>

### `isPrefix`

```sml
val isPrefix : string -> substring -> bool
```

### `isSubstring`

```sml
val isSubstring : string -> substring -> bool
```

### `isSuffix`

```sml
val isSuffix : string -> substring -> bool
```

`isPrefix ``s`` ``ss`` `
` isSubstring ``s`` ``ss`` `
` isSuffix ``s`` ``ss`` `  
These functions return `true` if the string `s` is a prefix, substring, or suffix (respectively) of the substring `ss`. The functions are equivalent to their versions from [`STRING`](string.md#STRING:SIG:SPEC). For example, `isPrefix ``s`` ``ss` is the same as `String.isPrefix ``s`` (`[`string`](substring.md#SIG:SUBSTRING.string:VAL:SPEC)` ``ss``)`.



```repl
Substring.isPrefix "ab" (Substring.full "abc");; (* true *)
```

```repl
Substring.isSubstring "bc" (Substring.full "abc");; (* true *)
```

```repl
Substring.isSuffix "bc" (Substring.full "abc");; (* true *)
```

<span id="SIG:SUBSTRING.compare:VAL"></span>

### `compare`

```sml
val compare : substring * substring -> order
```

`compare (``s``, ``t``) `  
compares the two substrings lexicographically using the default character comparison function. This is equivalent to

[String.compare](string.md#SIG:STRING.compare:VAL:SPEC) ([string](substring.md#SIG:SUBSTRING.string:VAL:SPEC) `s`, [string](substring.md#SIG:SUBSTRING.string:VAL:SPEC) `t`)




```repl
Substring.compare (Substring.full "a", Substring.full "b");; (* LESS *)
```

<span id="SIG:SUBSTRING.collate:VAL"></span>

### `collate`

```sml
val collate : (char * char -> order) -> substring * substring -> order
```

`collate ``f`` (``s``, ``t``) `  
compares the two substrings lexicographically using the character comparison function `f`. This is equivalent to

[String.collate](string.md#SIG:STRING.collate:VAL:SPEC) `f` ([string](substring.md#SIG:SUBSTRING.string:VAL:SPEC) `s`, [string](substring.md#SIG:SUBSTRING.string:VAL:SPEC) `t`)




```repl
Substring.collate Char.compare (Substring.full "a", Substring.full "b");; (* LESS *)
```

<span id="SIG:SUBSTRING.splitl:VAL"></span>

### `splitl`

```sml
val splitl : (char -> bool) -> substring -> substring * substring
```

### `splitr`

```sml
val splitr : (char -> bool) -> substring -> substring * substring
```

`splitl ``f`` ``s`` `
` splitr ``f`` ``s`` `  
These functions scan `s` from left to right (respectively, right to left) looking for the first character that does not satisfy the predicate `f`. They return the pair `(``ls``, ``rs``)` giving the split of the substring into the span up to that character and the rest. `ls` is the left side of the split, and `rs` is the right side. For example, if the characters `a` and `c` satisfy the predicate, but character `X` does not, then these functions work as follows on the substring `aaaXbbbbXccc`:

[splitl](substring.md#SIG:SUBSTRING.splitl:VAL:SPEC)   :           aaa         XbbbbXccc
[splitr](substring.md#SIG:SUBSTRING.splitr:VAL:SPEC)   :           aaaXbbbbX   ccc




```repl
let val (a, b) = Substring.splitl Char.isAlpha (Substring.full "abc 123") in (Substring.string a, Substring.string b) end;; (* ("abc", " 123") *)
```

```repl
let val (a, b) = Substring.splitr Char.isAlpha (Substring.full "123 abc") in (Substring.string a, Substring.string b) end;; (* ("123 ", "abc") *)
```

<span id="SIG:SUBSTRING.splitAt:VAL"></span>

### `splitAt`

```sml
val splitAt : substring * int -> substring * substring
```

`splitAt (``s``, ``i``) `  
returns the pair of substring `(``ss``, ``ss'``)`, where `ss` contains the first `i` characters of `s` and `ss'` contains the rest, assuming 0 \<= `i` \<= `size` `s`. Otherwise, it raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC).



```repl
let val (a, b) = Substring.splitAt (Substring.full "abc", 1) in (Substring.string a, Substring.string b) end;; (* ("a", "bc") *)
```

<span id="SIG:SUBSTRING.dropl:VAL"></span>

### `dropl`

```sml
val dropl : (char -> bool) -> substring -> substring
```

### `dropr`

```sml
val dropr : (char -> bool) -> substring -> substring
```

### `takel`

```sml
val takel : (char -> bool) -> substring -> substring
```

### `taker`

```sml
val taker : (char -> bool) -> substring -> substring
```

`dropl ``f`` ``s`` `
` dropr ``f`` ``s`` `
` takel ``f`` ``s`` `
` taker ``f`` ``s`` `  
These routines scan the substring `s` for the first character not satisfying the predicate `p`. The functions [`dropl`](substring.md#SIG:SUBSTRING.dropl:VAL:SPEC) and [`takel`](substring.md#SIG:SUBSTRING.takel:VAL:SPEC) scan left to right (_i.e._, increasing character indices), while [`dropr`](substring.md#SIG:SUBSTRING.dropr:VAL:SPEC) and [`taker`](substring.md#SIG:SUBSTRING.taker:VAL:SPEC) scan from the right. The drop functions drop the maximal substring consisting of characters satisfying the predicate, while the take functions return the maximal such substring. These can be defined in terms of the split operations:

[takel](substring.md#SIG:SUBSTRING.takel:VAL:SPEC) `p` `s` = #1([splitl](substring.md#SIG:SUBSTRING.splitl:VAL:SPEC) `p` `s`)
[dropl](substring.md#SIG:SUBSTRING.dropl:VAL:SPEC) `p` `s` = #2([splitl](substring.md#SIG:SUBSTRING.splitl:VAL:SPEC) `p` `s`)
[taker](substring.md#SIG:SUBSTRING.taker:VAL:SPEC) `p` `s` = #2([splitr](substring.md#SIG:SUBSTRING.splitr:VAL:SPEC) `p` `s`)
[dropr](substring.md#SIG:SUBSTRING.dropr:VAL:SPEC) `p` `s` = #1([splitr](substring.md#SIG:SUBSTRING.splitr:VAL:SPEC) `p` `s`)




```repl
Substring.string (Substring.dropl Char.isAlpha (Substring.full "abc 123"));; (* " 123" *)
```

```repl
Substring.string (Substring.dropr Char.isAlpha (Substring.full "123 abc"));; (* "123 " *)
```

```repl
Substring.string (Substring.takel Char.isAlpha (Substring.full "abc 123"));; (* "abc" *)
```

```repl
Substring.string (Substring.taker Char.isAlpha (Substring.full "123 abc"));; (* "abc" *)
```

<span id="SIG:SUBSTRING.position:VAL"></span>

### `position`

```sml
val position : string -> substring -> substring * substring
```

`position ``s`` ``ss`` `  
splits the substring `ss` into a pair `(``pref``, ``suff``)` of substrings, where `suff` is the longest suffix of `ss` that has `s` as a prefix and `pref` is the prefix of `ss` preceding `suff`. More precisely, let `m` be the size of `s` and let `ss` correspond to the substring `(``s'``, ``i``, ``n``)`. If there is a least index `k` \>= `i` such that `s`` = ``s'``[``k``..``k``+``m``-1]`, then `suff` corresponds to `(``s'``, ``k``, ``n``+``i``-``k``)` and `pref` corresponds to `(``s'``, ``i``, ``k``-``i``)`. If there is no such `k`, then `suff` is the empty substring corresponding to `(``s'``, ``i``+``n``, 0)` and `pref` corresponds to `(``s'``, ``i``, ``n``)`, _i.e._, all of `ss`.



```repl
let val (a, b) = Substring.position "bc" (Substring.full "abcd") in (Substring.string a, Substring.string b) end;; (* ("a", "bcd") *)
```

<span id="SIG:SUBSTRING.span:VAL"></span>

### `span`

```sml
val span : substring * substring -> substring
```

`span (``ss``, ``ss'``) `  
produces a substring composed of a prefix `ss`, suffix `ss'`, plus all intermediate characters in the underlying string. It raises [`Span`](general.md#SIG:GENERAL.Span:EXN:SPEC) if `ss` and `ss'` are not substrings of the same underlying string or if the start of `ss` is to the right of the end of `ss'`. More precisely, if we have

val (s, i, n) = base `ss`
val (s', i', n') = base `ss'`

then `span` returns `substring(s, i, (i'+n')-i)` unless `s <> s'` or `i'+n' < i`, in which case it raises [`Span`](general.md#SIG:GENERAL.Span:EXN:SPEC). Note that this does not preclude `ss'` from beginning to the left of `ss`, or `ss` from ending to the right of `ss'`.

This function allows one to scan for a substring using multiple pieces and then coalescing the pieces. For example, given a URL string such as

"http://www.standardml.org/Basis/overview.html"

to scan the protocol and host (`"http://www.standardml.org"`), one could write:

local
  open Substring
in
  fun protoAndHost url = let
        fun notc (c : char) = fn c' =\> c \<\> c'
        val (proto,rest) = splitl (notc #":") (full url)
        val host = takel (notc #"/") (triml 3 rest)
        in
          span (proto, host)
        end
end

> **Implementation note:**
>
> When applied to substrings derived from the identical base string, the string equality test should be constant time. This can be achieved by first doing a pointer test and, only if that fails, then checking the strings character by character.




```repl
let val s = "abc" in Substring.string (Substring.span (Substring.substring (s, 0, 1), Substring.substring (s, 2, 1))) end;; (* "abc" *)
```

<span id="SIG:SUBSTRING.translate:VAL"></span>

### `translate`

```sml
val translate : (char -> string) -> substring -> string
```

`translate ``f`` ``s`` `  
applies `f` to every character of `s`, from left to right, and returns the concatenation of the results. This is equivalent to [`String.concat`](string.md#SIG:STRING.concat:VAL:SPEC)`(`[`List.map`](list.md#SIG:LIST.map:VAL:SPEC)` ``f`` (`[`explode`](substring.md#SIG:SUBSTRING.explode:VAL:SPEC)` ``s``))`.



```repl
Substring.translate (String.str o Char.toUpper) (Substring.full "abc");; (* "ABC" *)
```

<span id="SIG:SUBSTRING.tokens:VAL"></span>

### `tokens`

```sml
val tokens : (char -> bool) -> substring -> substring list
```

### `fields`

```sml
val fields : (char -> bool) -> substring -> substring list
```

`tokens ``f`` ``s`` `
` fields ``f`` ``s`` `  
These functions decompose a substring into a list of tokens or fields from left to right. A token is a non-empty maximal substring not containing any delimiter. A field is a (possibly empty) maximal substring of `s` not containing any delimiter. In both cases, a delimiter is a character satisfying predicate `f`.

Two tokens may be separated by more than one delimiter, whereas two fields are separated by exactly one delimiter. For example, if the only delimiter is the character `#"|"`, then the substring `"|abc||def"` contains two tokens `"abc"` and `"def"`, whereas it contains the four fields `""`, `"abc"`, `""` and `"def"`.



```repl
List.map Substring.string (Substring.tokens (fn c => c = #"|") (Substring.full "|a||b"));; (* ["a", "b"] *)
```

```repl
List.map Substring.string (Substring.fields (fn c => c = #"|") (Substring.full "|a||b"));; (* ["", "a", "", "b"] *)
```

<span id="SIG:SUBSTRING.app:VAL"></span>

### `app`

```sml
val app : (char -> unit) -> substring -> unit
```

`app ``f`` ``s`` `  
applies `f` to each character of `s` from left to right. It is equivalent to [`List.app`](list.md#SIG:LIST.app:VAL:SPEC)` ``f`` (`[`explode`](substring.md#SIG:SUBSTRING.explode:VAL:SPEC)` ``s``)`.



```repl
Substring.app print (Substring.full "ok");; (* prints ok *)
```

<span id="SIG:SUBSTRING.foldl:VAL"></span>

### `foldl`

```sml
val foldl : (char * 'a -> 'a) -> 'a -> substring -> 'a
```

### `foldr`

```sml
val foldr : (char * 'a -> 'a) -> 'a -> substring -> 'a
```

`foldl ``f`` ``a`` ``s`` `
` foldr ``f`` ``a`` ``s`` `  
These fold the function `f` over the substring `s`, starting with the value `a`, from left to right and from right to left, respectively. They are the analogues of the identically named functions in [`List`](list.md#List:STR:SPEC). In particular, they are respectively equivalent to:

[List.foldl](list.md#SIG:LIST.foldl:VAL:SPEC) `f` `a` ([explode](substring.md#SIG:SUBSTRING.explode:VAL:SPEC) `s`)
[List.foldr](list.md#SIG:LIST.foldr:VAL:SPEC) `f` `a` ([explode](substring.md#SIG:SUBSTRING.explode:VAL:SPEC) `s`)




```repl
Substring.foldl (fn (c, n) => n + 1) 0 (Substring.full "abc");; (* 3 *)
```

```repl
Substring.foldr (fn (c, n) => n + 1) 0 (Substring.full "abc");; (* 3 *)
```

#### Examples

```repl
Substring.string (Substring.full "");; (* ""; empty substring *)
Substring.string (Substring.substring ("abc", 1, 2));; (* "bc" *)
```

#### See Also

> [`CHAR`](char.md#CHAR:SIG:SPEC), [`List`](list.md#List:STR:SPEC), [`STRING`](string.md#STRING:SIG:SPEC), [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC), [`TEXT`](text.md#TEXT:SIG:SPEC)

#### Discussion

> **Implementation note:**
>
> Functions that extract pieces of a substring, such as [`splitl`](substring.md#SIG:SUBSTRING.splitl:VAL:SPEC) or [`tokens`](substring.md#SIG:SUBSTRING.tokens:VAL:SPEC) must return substrings with the same base string. This requirement is particularly important if [`span`](substring.md#SIG:SUBSTRING.span:VAL:SPEC) is to be used to put the pieces back together again.
