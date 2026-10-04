# <span id="section:0"></span>The `PACK_WORD` signature

---

#### Synopsis

<span id="PACK_WORD:SIG:SPEC"></span>
<span id="PackWord{N}Big:STR:SPEC"></span>
<span id="PackWord{N}Little:STR:SPEC"></span>

```sml
signature PACK_WORD (* OPTIONAL *)
structure PackWord<N>Big :> PACK_WORD (* OPTIONAL *)
structure PackWord<N>Little :> PACK_WORD (* OPTIONAL *)
```

The `PackWord`_`<N>`_`Big` and `PackWord`_`<N>`_`Little` structures provide facilities for packing and unpacking N-bit word elements into [`Word8`](word.md#Word8:STR:SPEC) vectors. This mechanism allows word values to be transmitted in binary format over networks. The `PackWord`_`<N>`_`Big` structures perform big-endian packing and unpacking, while the `PackWord`_`<N>`_`Little` structures perform little-endian packing and unpacking.

---

#### Interface

<span id="SIG:PACK_WORD.bytesPerElem:VAL:SPEC"></span>
<span id="SIG:PACK_WORD.isBigEndian:VAL:SPEC"></span>
<span id="SIG:PACK_WORD.subVec:VAL:SPEC"></span>
<span id="SIG:PACK_WORD.subVecX:VAL:SPEC"></span>
<span id="SIG:PACK_WORD.subArr:VAL:SPEC"></span>
<span id="SIG:PACK_WORD.subArrX:VAL:SPEC"></span>
<span id="SIG:PACK_WORD.update:VAL:SPEC"></span>

```sml
val bytesPerElem : int
val isBigEndian : bool
val subVec : Word8Vector.vector * int -> LargeWord.word
val subVecX : Word8Vector.vector * int -> LargeWord.word
val subArr : Word8Array.array * int -> LargeWord.word
val subArrX : Word8Array.array * int -> LargeWord.word
val update : Word8Array.array * int * LargeWord.word -> unit
```

#### Description

<span id="SIG:PACK_WORD.bytesPerElem:VAL"></span>**`val`**` bytesPerElem `**`:`**` int`  
The number of bytes per element. Most implementations will provide several structures with values of [`bytesPerElem`](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC) that are small powers of two (_e.g._, 1, 2, 4, and 8, corresponding to N of 8, 16, 32, 64, respectively).

<span id="SIG:PACK_WORD.isBigEndian:VAL"></span>**`val`**` isBigEndian `**`:`**` bool`  
True if the structure implements a big-endian view of the data (most-significant byte first). Otherwise, the structure implements a little-endian view (least-significant byte first).

<span id="SIG:PACK_WORD.subVec:VAL"></span>
`subVec (``vec``, ``i``) `
`subVecX (``vec``, ``i``)`  
These extract the subvector

`vec`\[[bytesPerElem](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC)\*`i`..[bytesPerElem](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC)\*(`i`+1)-1\]

of the vector `vec` and convert it into a word according to the endianness of the structure. The [`subVecX`](pack-word.md#SIG:PACK_WORD.subVecX:VAL:SPEC) version extends the sign bit (most significant bit) when converting the subvector to a word. The functions raise the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if `i` \< 0 or if `Word8Vector.length` `vec` \< `bytesPerElem` \* (`i` + 1).

<span id="SIG:PACK_WORD.subArr:VAL"></span>
`subArr (``arr``, ``i``) `
`subArrX (``arr``, ``i``)`  
These extract the subarray

`arr`\[[bytesPerElem](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC)\*`i`..[bytesPerElem](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC)\*(`i`+1)-1\]

of the array `arr` and convert it into a word according to the endianness of the structure. The [`subArrX`](pack-word.md#SIG:PACK_WORD.subArrX:VAL:SPEC) version extends the sign bit (most significant bit) when converting the subarray into a word. The functions raise the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if `i` \< 0 or if [`Word8Array.length`](mono-array.md#SIG:MONO_ARRAY.length:VAL:SPEC) `arr` \< `bytesPerElem` \* (`i`+1).

<span id="SIG:PACK_WORD.update:VAL"></span>
`update (``arr``, ``i``, ``w``) `  
stores the [`bytesPerElem`](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC) low-order bytes of the word `w` into the bytes [`bytesPerElem`](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC)`*``i` through [`bytesPerElem`](pack-word.md#SIG:PACK_WORD.bytesPerElem:VAL:SPEC)`*(``i``+1)-1` of the array `arr`, according to the structure's endianness. It raises the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if `i` \< 0 or if `Word8Array.length` `arr` \< `bytesPerElem` \* (`i`+1).

#### Examples

```repl
PackWord32.toBytes 0w42;;
```

#### See Also

> [`Byte`](byte.md#Byte:STR:SPEC), [`LargeWord`](word.md#LargeWord:STR:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`PACK_REAL`](pack-float.md#PACK_REAL:SIG:SPEC)
