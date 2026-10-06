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

<span id="SIG:PACK_WORD.bytesPerElem:VAL"></span>

### `bytesPerElem`

```sml
val bytesPerElem : int
```
The number of bytes required to store one word value.

```repl
PackWord32.bytesPerElem;; (* 4 for a 32-bit word *)
```

<span id="SIG:PACK_WORD.isBigEndian:VAL"></span>

### `isBigEndian`

```sml
val isBigEndian : bool
```
Reports whether this structure uses big-endian byte order.

```repl
PackWord32.isBigEndian;; (* byte order used by this structure *)
```

<span id="SIG:PACK_WORD.subVec:VAL"></span>

### `subVec`

```sml
val subVec : Word8Vector.vector * int -> LargeWord.word
```
Reads `bytesPerElem` bytes at element index `i` and interprets them in this structure's byte order. It raises `Subscript` when the requested element is out of range.

```repl
PackWord32.subVec (Word8Vector.fromList [0w0, 0w0, 0w0, 0w42], 0);; (* 0w42 for big-endian byte order *)
```

<span id="SIG:PACK_WORD.subVecX:VAL"></span>

### `subVecX`

```sml
val subVecX : Word8Vector.vector * int -> LargeWord.word
```
Reads a vector element and sign extends its most significant bit.

```repl
PackWord32.subVecX (Word8Vector.fromList [0w0, 0w0, 0w0, 0w42], 0);; (* 0w42 for big-endian byte order *)
```

<span id="SIG:PACK_WORD.subArr:VAL"></span>

### `subArr`

```sml
val subArr : Word8Array.array * int -> LargeWord.word
```
Reads `bytesPerElem` bytes at element index `i` from the array.

```repl
let val a = Word8Array.array (PackWord32.bytesPerElem, 0w0); val _ = PackWord32.update (a, 0, 0w42) in PackWord32.subArr (a, 0) end;; (* 0w42 *)
```

<span id="SIG:PACK_WORD.subArrX:VAL"></span>

### `subArrX`

```sml
val subArrX : Word8Array.array * int -> LargeWord.word
```
Reads an array element and sign extends its most significant bit.

```repl
let val a = Word8Array.array (PackWord32.bytesPerElem, 0w0); val _ = PackWord32.update (a, 0, 0w42) in PackWord32.subArrX (a, 0) end;; (* 0w42 *)
```

<span id="SIG:PACK_WORD.update:VAL"></span>

### `update`

```sml
val update : Word8Array.array * int * LargeWord.word -> unit
```
Stores the low-order bytes of a word into the array at element index `i`.

```repl
let val a = Word8Array.array (PackWord32.bytesPerElem, 0w0) in PackWord32.update (a, 0, 0w42) end;; (* () *)
```

#### See Also

> [`Byte`](byte.md#Byte:STR:SPEC), [`LargeWord`](word.md#LargeWord:STR:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`PACK_REAL`](pack-float.md#PACK_REAL:SIG:SPEC)

```repl
let val a = Word8Array.array (PackWord32.bytesPerElem, 0w0); val _ = PackWord32.update (a, 0, 0w42) in PackWord32.subArr (a, 0) end;; (* 0w42 *)
```
