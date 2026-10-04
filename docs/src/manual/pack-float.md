# <span id="section:0"></span>The `PACK_REAL` signature

---

#### Synopsis

<span id="PACK_REAL:SIG:SPEC"></span>
<span id="PackRealBig:STR:SPEC"></span>
<span id="PackRealLittle:STR:SPEC"></span>
<span id="PackReal{N}Big:STR:SPEC"></span>
<span id="PackReal{N}Little:STR:SPEC"></span>

```sml
signature PACK_REAL (* OPTIONAL *)
structure PackRealBig :> PACK_REAL (* OPTIONAL *)
where type real = Real.real
structure PackRealLittle :> PACK_REAL (* OPTIONAL *)
where type real = Real.real
structure PackReal<N>Big :> PACK_REAL (* OPTIONAL *)
where type real = Real{N}.real
structure PackReal<N>Little :> PACK_REAL (* OPTIONAL *)
where type real = Real{N}.real
```

The `PACK_REAL` signature specifies the interface for packing and unpacking floating-point numbers into [`Word8`](word.md#Word8:STR:SPEC) vectors and arrays. This provides a mechanism for transmitting floating-point values over a network.

For each optional `Real`_`<N>`_ structure provided by an implementation, the implementation may also provide a pair of structures `PackReal`_`<N>`_`Big` and `PackReal`_`<N>`_`Little`. These structures share the real type defined in `Real`_`<N>`_. The `PackReal`_`<N>`_`Big` structures perform big-endian packing and unpacking, and the `PackReal`_`<N>`_`Little` structures perform little-endian packing and unpacking.

In addition, an implementation may provide the structures `PackRealBig` and `PackRealLittle`, which are aliases for the `PACK_REAL` structures related to the default [`Real`](real.md#Real:STR:SPEC) structure.

---

#### Interface

<span id="SIG:PACK_REAL.real:TY:SPEC"></span>
<span id="SIG:PACK_REAL.bytesPerElem:VAL:SPEC"></span>
<span id="SIG:PACK_REAL.isBigEndian:VAL:SPEC"></span>
<span id="SIG:PACK_REAL.toBytes:VAL:SPEC"></span>
<span id="SIG:PACK_REAL.fromBytes:VAL:SPEC"></span>
<span id="SIG:PACK_REAL.subVec:VAL:SPEC"></span>
<span id="SIG:PACK_REAL.subArr:VAL:SPEC"></span>
<span id="SIG:PACK_REAL.update:VAL:SPEC"></span>

```sml
type real
val bytesPerElem : int
val isBigEndian : bool
val toBytes : real -> Word8Vector.vector
val fromBytes : Word8Vector.vector -> real
val subVec : Word8Vector.vector * int -> real
val subArr : Word8Array.array * int -> real
val update : Word8Array.array * int * real -> unit
```

#### Description

<span id="SIG:PACK_REAL.bytesPerElem:VAL"></span>**`val`**` bytesPerElem `**`:`**` int`  
The number of bytes per element, sufficient to store a value of type [`real`](pack-float.md#SIG:PACK_REAL.real:TY:SPEC).

<span id="SIG:PACK_REAL.isBigEndian:VAL"></span>
`isBigEndian `  
is `true` if the structure implements a big-endian view of the data.

<span id="SIG:PACK_REAL.toBytes:VAL"></span>**`val`**` toBytes `**`:`**` real `**`->`**` Word8Vector.vector`
**`val`**` fromBytes `**`:`**` Word8Vector.vector `**`->`**` real`  
These functions pack and unpack floating-point values into and out of [`Word8Vector.vector`](mono-vector.md#SIG:MONO_VECTOR.vector:TY:SPEC) values. The function [`fromBytes`](pack-float.md#SIG:PACK_REAL.fromBytes:VAL:SPEC) raises the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if the argument vector does not have length at least [`bytesPerElem`](pack-float.md#SIG:PACK_REAL.bytesPerElem:VAL:SPEC); otherwise the first [`bytesPerElem`](pack-float.md#SIG:PACK_REAL.bytesPerElem:VAL:SPEC) bytes are used.

<span id="SIG:PACK_REAL.subVec:VAL"></span>
`subVec (``seq``, ``i``) `
`subArr (``seq``, ``i``)`  
These functions extract the subsequence

`seq`\[[bytesPerElem](pack-float.md#SIG:PACK_REAL.bytesPerElem:VAL:SPEC)\*`i`..[bytesPerElem](pack-float.md#SIG:PACK_REAL.bytesPerElem:VAL:SPEC)\*(`i`+1)-1\]

of the aggregate `seq` and convert it into a [`real`](pack-float.md#SIG:PACK_REAL.real:TY:SPEC) value according to the endianness of the structure. They raise the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if `i` \< 0 or if `Word8Array.length` `seq` \< `bytesPerElem` \* (`i` + 1).

<span id="SIG:PACK_REAL.update:VAL"></span>
`update (``arr``, ``i``, ``r``) `  
stores `r` into the bytes [`bytesPerElem`](pack-float.md#SIG:PACK_REAL.bytesPerElem:VAL:SPEC)`*``i` through [`bytesPerElem`](pack-float.md#SIG:PACK_REAL.bytesPerElem:VAL:SPEC)`*(``i``+1)-1` of the array `arr`, according to the structure's endianness. It raises the [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) exception if `i` \< 0 or if `Word8Array.length` `arr` \< `bytesPerElem` \* (`i` + 1).

#### Examples

```repl
PackReal64.toBytes 3.14;;
```

#### See Also

> [`PACK_WORD`](pack-word.md#PACK_WORD:SIG:SPEC), [`REAL`](real.md#REAL:SIG:SPEC)
