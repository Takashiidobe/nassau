# <span id="section:0"></span>The `PrimIO` functor

---

#### Synopsis

<span id="PrimIO:FCT:SPEC"></span>

```sml
functor PrimIO ( ... ) : PRIM_IO (* OPTIONAL *)
```

The optional functor `PrimIO` builds an instance of the primitive I/O signature [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC).

---

#### Functor argument interface

<span id="ARG:PrimIO.someElem:VAL:SPEC"></span>
<span id="ARG:PrimIO.pos:TY:SPEC"></span>
<span id="ARG:PrimIO.compare:VAL:SPEC"></span>

```sml
structure Vector : MONO_VECTOR
structure VectorSlice : MONO_VECTOR_SLICE
structure Array : MONO_ARRAY
structure ArraySlice : MONO_ARRAY_SLICE
sharing type Vector.elem = VectorSlice.elem = Array.elem
= ArraySlice.elem
sharing type Vector.vector = VectorSlice.vector
= Array.vector = ArraySlice.vector
sharing type VectorSlice.slice = ArraySlice.vector_slice
sharing type Array.array = ArraySlice.array
val someElem : Vector.elem
eqtype pos
val compare : pos * pos -> order
```

#### Description

<span id="ARG:PrimIO.someElem:VAL"></span>**`val`**` someElem `**`:`**` Vector.elem`  
An element that may be read or written by a [`reader`](prim-io.md#SIG:PRIM_IO.reader:TY:SPEC) or [`writer`](prim-io.md#SIG:PRIM_IO.writer:TY:SPEC). The value [`someElem`](prim-io-fn.md#ARG:PrimIO.someElem:VAL:SPEC) is typically used for initialization of buffers.

<span id="ARG:PrimIO.compare:VAL"></span>
`compare (``pos``, ``pos'``) `  
returns [`LESS`](general.md#SIG:GENERAL.order:TY:SPEC), [`EQUAL`](general.md#SIG:GENERAL.order:TY:SPEC), or [`GREATER`](general.md#SIG:GENERAL.order:TY:SPEC) when `pos` is less than, equal to, or greater than `pos'`, respectively, in some underlying linear ordering on [`pos`](prim-io-fn.md#ARG:PrimIO.pos:TY:SPEC) values.

#### Examples

```repl
TextIO.getInstream TextIO.stdIn;;
```

#### See Also

> [`General`](general.md#General:STR:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_ARRAY_SLICE`](mono-array-slice.md#MONO_ARRAY_SLICE:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`MONO_VECTOR_SLICE`](mono-vector-slice.md#MONO_VECTOR_SLICE:SIG:SPEC), [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC), [`StreamIO`](stream-io-fn.md#StreamIO:FCT:SPEC)
