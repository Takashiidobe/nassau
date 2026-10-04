# <span id="section:0"></span>The `Byte` structure

---

#### Synopsis

<span id="BYTE:SIG:SPEC"></span>
<span id="Byte:STR:SPEC"></span>

```sml
signature BYTE
structure Byte :> BYTE
```

Bytes are 8-bit integers as provided by the [`Word8`](word.md#Word8:STR:SPEC) structure, but serve the dual role as elements composing the extended ASCII character set. The `Byte` structure provides functions for converting values between these two roles.

---

#### Interface

<span id="SIG:BYTE.byteToChar:VAL:SPEC"></span>
<span id="SIG:BYTE.charToByte:VAL:SPEC"></span>
<span id="SIG:BYTE.bytesToString:VAL:SPEC"></span>
<span id="SIG:BYTE.stringToBytes:VAL:SPEC"></span>
<span id="SIG:BYTE.unpackStringVec:VAL:SPEC"></span>
<span id="SIG:BYTE.unpackString:VAL:SPEC"></span>
<span id="SIG:BYTE.packString:VAL:SPEC"></span>

```sml
val byteToChar : Word8.word -> char
val charToByte : char -> Word8.word
val bytesToString : Word8Vector.vector -> string
val stringToBytes : string -> Word8Vector.vector
val unpackStringVec : Word8VectorSlice.slice -> string
val unpackString : Word8ArraySlice.slice -> string
val packString : Word8Array.array * int * substring -> unit
```

#### Description

<span id="SIG:BYTE.byteToChar:VAL"></span>
`byteToChar ``i`` `  
returns the character whose code is `i`.

<span id="SIG:BYTE.charToByte:VAL"></span>
`charToByte ``c`` `  
returns an 8-bit word holding the code for the character `c`.

<span id="SIG:BYTE.bytesToString:VAL"></span>**`val`**` bytesToString `**`:`**` Word8Vector.vector `**`->`**` string`
**`val`**` stringToBytes `**`:`**` string `**`->`**` Word8Vector.vector`  
These functions convert between a vector of character codes and the corresponding string. Note that these functions do not perform end-of-line, or other character, translations. The semantics of these functions can be defined as follows, although one expects actual implementations will be more efficient:

        fun bytesToString bv =
          CharVector.tabulate(
                Word8Vector.length bv,
                fn i =\> byteToChar(Word8Vector.sub(bv, i)))
        fun stringToBytes s =
          Word8Vector.tabulate(
                String.size s,
                fn i =\> charToByte(String.sub(s, i)))

> **Implementation note:**
>
> For implementations where the underlying representation of the [`Word8Vector.vector`](mono-vector.md#SIG:MONO_VECTOR.vector:TY:SPEC) and [`string`](string.md#SIG:STRING.string:TY:SPEC) types are the same, these functions should be constant-time operations.


<span id="SIG:BYTE.unpackStringVec:VAL"></span>
`unpackStringVec ``slice`` `  
returns the string consisting of characters whose codes are held in the vector slice `slice`.

<span id="SIG:BYTE.unpackString:VAL"></span>**`val`**` unpackString `**`:`**` Word8ArraySlice.slice `**`->`**` string`  
returns the string consisting of characters whose codes are held in the array slice `slice`.

<span id="SIG:BYTE.packString:VAL"></span>
`packString (``arr``, ``i``, ``s``) `  
puts the substring `s` into the array `arr` starting at offset `i`. It raises [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if `i` \< 0 or `size` `s` + `i` \> \|`arr`\|.

#### Examples

```repl
Byte.bytesToString (Byte.stringToBytes "Nassau");;
```

#### See Also

> [`Char`](char.md#Char:STR:SPEC), [`String`](string.md#String:STR:SPEC), [`Substring`](substring.md#Substring:STR:SPEC), [`WORD`](word.md#WORD:SIG:SPEC), [`Word8`](word.md#Word8:STR:SPEC), [`Word8Vector`](mono-vector.md#Word8Vector:STR:SPEC), [`Word8VectorSlice`](mono-vector-slice.md#Word8VectorSlice:STR:SPEC), [`Word8Array`](mono-array.md#Word8Array:STR:SPEC), [`Word8ArraySlice`](mono-array-slice.md#Word8ArraySlice:STR:SPEC)
