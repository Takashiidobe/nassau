# <span id="section:0"></span>The `WORD` signature

---

#### Synopsis

<span id="WORD:SIG:SPEC"></span>
<span id="Word:STR:SPEC"></span>
<span id="Word8:STR:SPEC"></span>
<span id="LargeWord:STR:SPEC"></span>
<span id="Word{N}:STR:SPEC"></span>
<span id="SysWord:STR:SPEC"></span>

```sml
signature WORD
structure Word :> WORD
where type word = word
structure Word8 :> WORD
structure LargeWord :> WORD
structure Word<N> :> WORD (* OPTIONAL *)
structure SysWord :> WORD (* OPTIONAL *)
```

Instances of the signature `WORD` provide a type of unsigned integer with modular arithmetic and logical operations and conversion operations. They are also meant to give efficient access to the primitive machine word types of the underlying hardware, and support bit-level operations on integers. They are not meant to be a \`\`larger'' [`int`](integer.md#SIG:INTEGER.int:TY:SPEC).

In order to provide a more intuitive description of the shift operators below, we assume a bit ordering in which the most significant bit is leftmost, and the least significant bit is rightmost.

---

#### Interface

<span id="SIG:WORD.word:TY:SPEC"></span>
<span id="SIG:WORD.wordSize:VAL:SPEC"></span>
<span id="SIG:WORD.toLarge:VAL:SPEC"></span>
<span id="SIG:WORD.toLargeX:VAL:SPEC"></span>
<span id="SIG:WORD.toLargeWord:VAL:SPEC"></span>
<span id="SIG:WORD.toLargeWordX:VAL:SPEC"></span>
<span id="SIG:WORD.fromLarge:VAL:SPEC"></span>
<span id="SIG:WORD.fromLargeWord:VAL:SPEC"></span>
<span id="SIG:WORD.toLargeInt:VAL:SPEC"></span>
<span id="SIG:WORD.toLargeIntX:VAL:SPEC"></span>
<span id="SIG:WORD.fromLargeInt:VAL:SPEC"></span>
<span id="SIG:WORD.toInt:VAL:SPEC"></span>
<span id="SIG:WORD.toIntX:VAL:SPEC"></span>
<span id="SIG:WORD.fromInt:VAL:SPEC"></span>
<span id="SIG:WORD.andb:VAL:SPEC"></span>
<span id="SIG:WORD.orb:VAL:SPEC"></span>
<span id="SIG:WORD.xorb:VAL:SPEC"></span>
<span id="SIG:WORD.notb:VAL:SPEC"></span>
<span id="SIG:WORD.\|@LT\|\|@LT\|:VAL:SPEC"></span>
<span id="SIG:WORD.\|@GT\|\|@GT\|:VAL:SPEC"></span>
<span id="SIG:WORD.~\|@GT\|\|@GT\|:VAL:SPEC"></span>
<span id="SIG:WORD.+:VAL:SPEC"></span>
<span id="SIG:WORD.-:VAL:SPEC"></span>
<span id="SIG:WORD.*:VAL:SPEC"></span>
<span id="SIG:WORD.div:VAL:SPEC"></span>
<span id="SIG:WORD.mod:VAL:SPEC"></span>
<span id="SIG:WORD.compare:VAL:SPEC"></span>
<span id="SIG:WORD.\|@LT\|:VAL:SPEC"></span>
<span id="SIG:WORD.\|@LTE\|:VAL:SPEC"></span>
<span id="SIG:WORD.\|@GT\|:VAL:SPEC"></span>
<span id="SIG:WORD.\|@GTE\|:VAL:SPEC"></span>
<span id="SIG:WORD.~:VAL:SPEC"></span>
<span id="SIG:WORD.min:VAL:SPEC"></span>
<span id="SIG:WORD.max:VAL:SPEC"></span>
<span id="SIG:WORD.fmt:VAL:SPEC"></span>
<span id="SIG:WORD.toString:VAL:SPEC"></span>
<span id="SIG:WORD.scan:VAL:SPEC"></span>
<span id="SIG:WORD.fromString:VAL:SPEC"></span>

```sml
eqtype word
val wordSize : int
val toLarge : word -> LargeWord.word
val toLargeX : word -> LargeWord.word
val toLargeWord : word -> LargeWord.word
val toLargeWordX : word -> LargeWord.word
val fromLarge : LargeWord.word -> word
val fromLargeWord : LargeWord.word -> word
val toLargeInt : word -> LargeInt.int
val toLargeIntX : word -> LargeInt.int
val fromLargeInt : LargeInt.int -> word
val toInt : word -> int
val toIntX : word -> int
val fromInt : int -> word
val andb : word * word -> word
val orb : word * word -> word
val xorb : word * word -> word
val notb : word -> word
val << : word * Word.word -> word
val >> : word * Word.word -> word
val ~>> : word * Word.word -> word
val + : word * word -> word
val - : word * word -> word
val * : word * word -> word
val div : word * word -> word
val mod : word * word -> word
val compare : word * word -> order
val < : word * word -> bool
val <= : word * word -> bool
val > : word * word -> bool
val >= : word * word -> bool
val ~ : word -> word
val min : word * word -> word
val max : word * word -> word
val fmt : StringCvt.radix -> word -> string
val toString : word -> string
val scan : StringCvt.radix -> (char, 'a) StringCvt.reader -> (word, 'a) StringCvt.reader
val fromString : string -> word option
```

#### Description

<span id="SIG:WORD.wordSize:VAL"></span>**`val`**` wordSize `**`:`**` int`  
The number of bits in type [`word`](word.md#SIG:WORD.word:TY:SPEC). [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) need not be a power of two. Note that [`word`](word.md#SIG:WORD.word:TY:SPEC) has a fixed, finite precision.

<span id="SIG:WORD.toLarge:VAL"></span>
`toLarge ``w`` `
` toLargeX ``w`` `  
These convert `w` to a value of type [`LargeWord.word`](word.md#SIG:WORD.word:TY:SPEC). In the first case, `w` is converted to its equivalent [`LargeWord.word`](word.md#SIG:WORD.word:TY:SPEC) value in the range \[0,2<sup>(`wordSize`)</sup>-1\]. In the second case, `w` is \`\`sign-extended,'' _i.e._, the [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) low-order bits of `w` and [`toLargeX`](word.md#SIG:WORD.toLargeX:VAL:SPEC)` ``w` are the same, and the remaining bits of [`toLargeX`](word.md#SIG:WORD.toLargeX:VAL:SPEC)` ``w` are all equal to the most significant bit of `w`.

[`toLargeWord`](word.md#SIG:WORD.toLargeWord:VAL:SPEC) and [`toLargeWordX`](word.md#SIG:WORD.toLargeWordX:VAL:SPEC) are respective synonyms of the first two, and are deprecated.

<span id="SIG:WORD.fromLarge:VAL"></span>
`fromLarge ``w`` `
` fromLargeWord ``w`` `  
These functions convert `w` to the value `w`(**mod** (2<sup>(`wordSize`)</sup>)) of type [`word`](word.md#SIG:WORD.word:TY:SPEC). This has the effect of taking the low-order [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) bits of the 2's complement representation of `w`.

[`fromLargeWord`](word.md#SIG:WORD.fromLargeWord:VAL:SPEC) is a deprecated synonym for [`fromLarge`](word.md#SIG:WORD.fromLarge:VAL:SPEC).

<span id="SIG:WORD.toLargeInt:VAL"></span>
`toLargeInt ``w`` `
` toLargeIntX ``w`` `  
These convert `w` to a value of type [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC). In the former case, `w` is viewed as an integer value in the range \[0,2<sup>(`wordSize`)</sup>-1\]. In the latter case, `w` is treated as a 2's complement signed integer with [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) precision, thereby having a value in the range \[-2<sup>(`wordSize`-1)</sup>,2<sup>(`wordSize`-1)</sup>-1\]. [`toLargeInt`](word.md#SIG:WORD.toLargeInt:VAL:SPEC) raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the target integer value cannot be represented as a [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC). Since the precision of [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC) is always at least [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) (see the discussion below), [`toLargeIntX`](word.md#SIG:WORD.toLargeIntX:VAL:SPEC) will never raise an exception.

<span id="SIG:WORD.fromLargeInt:VAL"></span>
`fromLargeInt ``i`` `  
converts `i` of type [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC) to a value of type [`word`](word.md#SIG:WORD.word:TY:SPEC). This has the effect of taking the low-order [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) bits of the 2's complement representation of `i`.

<span id="SIG:WORD.toInt:VAL"></span>
`toInt ``w`` `
` toIntX ``w`` `  
These convert `w` to a value of default integer type. In the former case, `w` is viewed as an integer value in the range \[0,2<sup>(`wordSize`)</sup>-1\]. In the latter case, `w` is treated as a 2's complement signed integer with [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) precision, thereby having a value in the range \[-2<sup>(`wordSize`-1)</sup>,2<sup>(`wordSize`-1)</sup>-1\]. They raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the target integer value cannot be represented as an [`Int.int`](integer.md#SIG:INTEGER.int:TY:SPEC).

<span id="SIG:WORD.fromInt:VAL"></span>
`fromInt ``i`` `  
converts `i` of the default integer type to a value of type [`word`](word.md#SIG:WORD.word:TY:SPEC). This has the effect of taking the low-order [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) bits of the 2's complement representation of `i`. If the precision of [`Int.int`](integer.md#SIG:INTEGER.int:TY:SPEC) is less than [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC), then `i` is sign-extended to [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) bits.

<span id="SIG:WORD.andb:VAL"></span>**`val`**` andb `**`:`**` word `**`*`**` word `**`->`**` word`
**`val`**` orb `**`:`**` word `**`*`**` word `**`->`**` word`
**`val`**` xorb `**`:`**` word `**`*`**` word `**`->`**` word`  
These functions return the bit-wise AND, OR, and exclusive OR, respectively, of their arguments.

<span id="SIG:WORD.notb:VAL"></span>
`notb ``i`` `  
returns the bit-wise complement (NOT) of `i`.

<span id="SIG:WORD.\|@LT\|\|@LT\|:VAL"></span>
`<< (``i``, ``n``) `  
shifts `i` to the left by `n` bit positions, filling in zeros from the right. When `i` and `n` are interpreted as unsigned binary numbers, this returns (`i`\* 2<sup>(`n`)</sup>)(**mod** (2 <sup>(`wordSize`)</sup>)). In particular, shifting by greater than or equal to [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) results in 0. This operation is similar to the \`\`(logical) shift left'' instruction in many processors.

<span id="SIG:WORD.\|@GT\|\|@GT\|:VAL"></span>
`>> (``i``, ``n``) `  
shifts `i` to the right by `n` bit positions, filling in zeros from the left. When `i` and `n` are interpreted as unsigned binary numbers, it returns **floor**((`i` / 2<sup>(`n`)</sup>)). In particular, shifting by greater than or equal to [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) results in 0. This operation is similar to the \`\`logical shift right'' instruction in many processors.

<span id="SIG:WORD.~\|@GT\|\|@GT\|:VAL"></span>
`~>> (``i``, ``n``) `  
shifts `i` to the right by `n` bit positions. The value of the leftmost bit of `i` remains the same; in a 2's-complement interpretation, this corresponds to sign extension. When `i` is interpreted as a [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC)-bit 2's-complement integer and `n` is interpreted as an unsigned binary number, it returns **floor**((`i` / 2<sup>(`n`)</sup>)). In particular, shifting by greater than or equal to [`wordSize`](word.md#SIG:WORD.wordSize:VAL:SPEC) results in either 0 or all 1's. This operation is similar to the \`\`arithmetic shift right'' instruction in many processors.

<span id="SIG:WORD.+:VAL"></span>
`i`` + ``j`` `  
returns (`i`+`j`)(**mod** (2 <sup>(`wordSize`)</sup>)) when `i` and `j` are interpreted as unsigned binary numbers. It does _not_ raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC).

<span id="SIG:WORD.-:VAL"></span>
`i`` - ``j`` `  
returns the difference of `i` and `j` modulo (2<sup>(`wordSize`)</sup>):

> (2<sup>(`wordSize`)</sup> + `i` - `j`)(**mod** (2<sup>(`wordSize`)</sup>))

when `i` and `j` are interpreted as unsigned binary numbers. It does _not_ raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC).

<span id="SIG:WORD.*:VAL"></span>
`i`` * ``j`` `  
returns the product (`i`\*`j`)(**mod** (2<sup>(`wordSize`)</sup>)) when `i` and `j` are interpreted as unsigned binary numbers. It does _not_ raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC).

<span id="SIG:WORD.div:VAL"></span>
`i`` div ``j`` `  
returns the truncated quotient of `i` and `j`, **floor**((`i` / `j`)), when `i` and `j` are interpreted as unsigned binary numbers. It raises [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) when `j` = 0.

<span id="SIG:WORD.mod:VAL"></span>
`i`` mod ``j`` `  
returns the remainder of the division of `i` by `j`:

> `i` - `j` \* **floor**((`i` / `j`))

when `i` and `j` are interpreted as unsigned binary numbers. It raises [`Div`](general.md#SIG:GENERAL.Div:EXN:SPEC) when `j` = 0.

<span id="SIG:WORD.compare:VAL"></span>
`compare (``i``, ``j``) `  
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) if and only if `i` is less than, equal to, or greater than `j`, respectively, considered as unsigned binary numbers.

<span id="SIG:WORD.\|@LT\|:VAL"></span>**`val`**` < `**`:`**` word `**`*`**` word `**`->`**` bool`
**`val`**` <= `**`:`**` word `**`*`**` word `**`->`**` bool`
**`val`**` > `**`:`**` word `**`*`**` word `**`->`**` bool`
**`val`**` >= `**`:`**` word `**`*`**` word `**`->`**` bool`  
These return `true` if and only if the input arguments satisfy the given relation when interpreted as unsigned binary numbers.

<span id="SIG:WORD.~:VAL"></span>
`~ ``i`` `  
returns the 2's complement of `i`.

<span id="SIG:WORD.min:VAL"></span>**`val`**` min `**`:`**` word `**`*`**` word `**`->`**` word`
**`val`**` max `**`:`**` word `**`*`**` word `**`->`**` word`  
These return the smaller (respectively, larger) of the arguments.

<span id="SIG:WORD.fmt:VAL"></span>
`fmt ``radix`` ``i`` `
` toString ``i`` `  
These return a string containing a numeric representation of `i`. No prefix `"Ow"`, `"OwX"`, etc. is generated. The version using [`fmt`](word.md#SIG:WORD.fmt:VAL:SPEC) creates a representation specified the given `radix`. The hexadecimal digits in the range \[10,15\] are represented by the characters `#"A"` through `#"F"`. The version using [`toString`](word.md#SIG:WORD.toString:VAL:SPEC) is equivalent to [`fmt`](word.md#SIG:WORD.fmt:VAL:SPEC)` `[`StringCvt.HEX`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)` ``i`.

<span id="SIG:WORD.scan:VAL"></span>
`scan ``radix`` ``getc`` ``strm`` `
` fromString ``s`` `  
These functions scan a [`word`](word.md#SIG:WORD.word:TY:SPEC) from a character source. In the first version, if an unsigned number in the format denoted by `radix` can be parsed from a prefix of the character strm `strm` using the character input function `getc`, the expression evaluates to [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(w,rest)`, where `w` is the value of the number parsed and `rest` is the remainder of the character stream. Initial whitespace is ignored. [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned otherwise. It raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when a number can be parsed, but is too large to fit in type `word`.

The format that `scan` accepts depends on the `radix` argument. Regular expressions defining these formats are as follows:

---

**Radix**

**Format**

[`StringCvt.BIN`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

(`0w`)<sup>?</sup>\[`0`-`1`\]<sup>+</sup>

[`StringCvt.OCT`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

(`0w`)<sup>?</sup>\[`0`-`7`\]<sup>+</sup>

[`StringCvt.DEC`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

(`0w`)<sup>?</sup>\[`0`-`9`\]<sup>+</sup>

[`StringCvt.HEX`](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC)

(`0wx` \| `0wX` \| `0x` \| `0X`)<sup>?</sup>\[`0`-`9``a`-`f``A`-`F`\]<sup>+</sup>

---

The [`fromString`](word.md#SIG:WORD.fromString:VAL:SPEC) version returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``w``)` if an unsigned hexadecimal number in the format (`0wx` \| `0wX` \| `0x` \| `0X`)<sup>?</sup>\[`0`-`9``a`-`f``A`-`F`\]<sup>+</sup> can be parsed from a prefix of string `s`, ignoring initial whitespace, where `w` is the value of the number parsed. [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned otherwise. This function raises [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) when a hexadecimal numeral can be parsed, but is too large to be represented by type [`word`](word.md#SIG:WORD.word:TY:SPEC). It is equivalent to

        [StringCvt.scanString](string-cvt.md#SIG:STRING_CVT.scanString:VAL:SPEC) (scan [StringCvt.HEX](string-cvt.md#SIG:STRING_CVT.radix:TY:SPEC))


#### Examples

```repl
Word.toString 0w42;;
```

#### See Also

> [`Byte`](byte.md#Byte:STR:SPEC), [`Int`](integer.md#Int:STR:SPEC), [`LargeInt`](integer.md#LargeInt:STR:SPEC), [`StringCvt`](string-cvt.md#StringCvt:STR:SPEC)

#### Discussion

A structure [`Word`_`<N>`_](word.md#Word%7BN%7D:STR:SPEC) implements `N`-bit words. The type [`LargeWord.word`](word.md#SIG:WORD.word:TY:SPEC) represents the largest word supported. We require that

> `LargeWord.wordSize` \<= `LargeInt.precision`

If [`LargeWord`](word.md#LargeWord:STR:SPEC) is not the same as [`Word`](word.md#Word:STR:SPEC), then there must be a structure [`Word`_`<N>`_](word.md#Word%7BN%7D:STR:SPEC) equal to [`LargeWord`](word.md#LargeWord:STR:SPEC).

The structure [`SysWord`](word.md#SysWord:STR:SPEC) is used with the optional [`Posix`](posix.md#Posix:STR:SPEC) and [`Windows`](windows.md#Windows:STR:SPEC) modules. The type [`SysWord.word`](word.md#SIG:WORD.word:TY:SPEC) is guaranteed to be large enough to hold any unsigned integral value used by the underlying system.

For words and integers of the same precision/word size, the operations [`fromInt`](word.md#SIG:WORD.fromInt:VAL:SPEC) and [`toIntX`](word.md#SIG:WORD.toIntX:VAL:SPEC) act as bit-wise identity functions. Even in this case, however, [`toInt`](word.md#SIG:WORD.toInt:VAL:SPEC) will raise [`Overflow`](general.md#SIG:GENERAL.Overflow:EXN:SPEC) if the high-order bit of the word is set.

Note that operations on words, and conversions of integral types into words, never cause exceptions to arise due to lost precision.

Conversion between words and integers of any size can be handled by intermediate conversion into [`LargeWord.word`](word.md#SIG:WORD.word:TY:SPEC) and [`LargeInt.int`](integer.md#SIG:INTEGER.int:TY:SPEC). For example, the functions [`fromInt`](word.md#SIG:WORD.fromInt:VAL:SPEC), [`toInt`](word.md#SIG:WORD.toInt:VAL:SPEC) and [`toIntX`](word.md#SIG:WORD.toIntX:VAL:SPEC) are respectively equivalent to:

fromLargeWord o LargeWord.fromLargeInt o Int.toLarge
Int.fromLarge o LargeWord.toLargeInt   o toLargeWord
Int.fromLarge o LargeWord.toLargeIntX  o toLargeWordX

Typically, implementations will provide very efficient word operations by expanding them inline to a few machine instructions. It also is assumed that implementations will catch the idiom of converting between words and integers of differing precisions using an intermediate representation (_e.g._, `Word32.fromLargeWord o Word8.toLargeWord`) and optimize these conversions.
