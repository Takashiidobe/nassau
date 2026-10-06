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

<span id="SIG:PACK_REAL.real:TY"></span>**`type`**` real`  
The floating-point type packed by this structure.

<span id="SIG:PACK_REAL.bytesPerElem:VAL"></span>

### `bytesPerElem`

```sml
val bytesPerElem : int
```
The number of bytes required to store one real value.

```repl
PackReal64.bytesPerElem;; (* 8 for a 64-bit real *)
```

<span id="SIG:PACK_REAL.isBigEndian:VAL"></span>

### `isBigEndian`

```sml
val isBigEndian : bool
```
Reports whether this structure uses big-endian byte order.

```repl
PackReal64.isBigEndian;; (* byte order used by this structure *)
```

<span id="SIG:PACK_REAL.toBytes:VAL"></span>

### `toBytes`

```sml
val toBytes : real -> Word8Vector.vector
```
Packs a real value into a byte vector.

```repl
PackReal64.toBytes 3.14;; (* packed bytes for 3.14 *)
```

<span id="SIG:PACK_REAL.fromBytes:VAL"></span>

### `fromBytes`

```sml
val fromBytes : Word8Vector.vector -> real
```
Unpacks a real value from the first `bytesPerElem` bytes. It raises `Subscript` if the vector is too short.

```repl
PackReal64.fromBytes (PackReal64.toBytes 3.14);; (* 3.14 *)
PackReal64.fromBytes (Word8Vector.fromList []);; (* raises Subscript *)
```

<span id="SIG:PACK_REAL.subVec:VAL"></span>

### `subVec`

```sml
val subVec : Word8Vector.vector * int -> real
```
Unpacks the value at element index `i` of the byte vector.

```repl
PackReal64.subVec (PackReal64.toBytes 3.14, 0);; (* 3.14 *)
```

<span id="SIG:PACK_REAL.subArr:VAL"></span>

### `subArr`

```sml
val subArr : Word8Array.array * int -> real
```
Unpacks the value at element index `i` of the byte array.

```repl
let val a = Word8Array.array (PackReal64.bytesPerElem, 0w0); val _ = PackReal64.update (a, 0, 3.14) in PackReal64.subArr (a, 0) end;; (* 3.14 *)
```

<span id="SIG:PACK_REAL.update:VAL"></span>

### `update`

```sml
val update : Word8Array.array * int * real -> unit
```
Packs the real value into the array beginning at element index `i`.

```repl
let val a = Word8Array.array (PackReal64.bytesPerElem, 0w0) in PackReal64.update (a, 0, 3.14) end;; (* () *)
```

#### See Also

> [`PACK_WORD`](pack-word.md#PACK_WORD:SIG:SPEC), [`REAL`](real.md#REAL:SIG:SPEC)

```repl
let val a = Word8Array.array (PackReal64.bytesPerElem, 0w0); val _ = PackReal64.update (a, 0, 3.14) in PackReal64.subArr (a, 0) end;; (* 3.14 *)
```
