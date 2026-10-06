# <span id="section:0"></span>The `StreamIO` functor

---

#### Synopsis

<span id="StreamIO:FCT:SPEC"></span>

```sml
functor StreamIO ( ... ) : STREAM_IO (* OPTIONAL *)
```

The optional `StreamIO` functor provides a way to build a Stream I/O layer on top of an arbitrary Primitive I/O implementation. For example, given an implementation of readers and writers for pairs of integers, one can define streams of pairs of integers.

---

#### Functor argument interface

<span id="ARG:StreamIO.someElem:VAL:SPEC"></span>

```sml
structure PrimIO : PRIM_IO
structure Vector : MONO_VECTOR
structure Array : MONO_ARRAY
sharing type PrimIO.elem = Vector.elem = Array.elem
sharing type PrimIO.vector = Vector.vector = Array.vector
sharing type PrimIO.array = Array.array
val someElem : PrimIO.elem
```

#### Description

<span id="ARG:StreamIO.PrimIO:STR"></span>**`structure`**` PrimIO `**`:`**` `[`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC)  
The underlying primitive I/O structure.

<span id="ARG:StreamIO.someElem:VAL"></span>

### `someElem`

```sml
val someElem : PrimIO.elem
```

Some arbitrary element used to initialize buffer arrays.

```repl
TextPrimIO.someElem;; (* representative character element *)
```

#### Examples

```repl
TextPrimIO.someElem;; (* representative character element *)
```

#### See Also

> [`ImperativeIO`](imperative-io-fn.md#ImperativeIO:FCT:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`PrimIO`](prim-io-fn.md#PrimIO:FCT:SPEC), [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC), [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC)

#### Discussion

The `Vector` and `Array` structures provide vector and array operations for manipulating the vectors and arrays used in `PrimIO` and `StreamIO`. The element `someElem` is used to initialize buffer arrays; any element will do.

The types `instream` and `outstream` in the result of the [`StreamIO`](stream-io-fn.md#StreamIO:FCT:SPEC) functor must be abstract.

> **Implementation note:**
>
> Here are some suggestions for efficient performance:
>
> - Operations on the underlying readers and writers (`readVec`, etc.) are expected to be expensive (involving a system call, with context switch).
> - Small input operations can be done from a buffer; the `readVec` or `readVecNB` operation of the underlying reader can replenish the buffer when necessary.
> - Each reader may provide only a subset of `readVec`, `readVecNB`, `block`, `canInput`, etc. An augmented reader that provides more operations can be constructed using `PrimIO.augmentReader`, but it may be more efficient to use the functions directly provided by the reader, instead of relying on the constructed ones. The same applies to augmented writers.
> - Keep the position of the beginning of the buffer on a multiple-of-`chunkSize` boundary, and do **read** or **write** operations with a multiple-of-`chunkSize` number of elements.
> - For very large `inputAll` or `inputN` operations, it is (somewhat) inefficient to read one `chunkSize` at a time and then concatenate all the results together. Instead, it is good to try to do the read all in one large system call; that is, `readVec(``n``)`. In a typical implementation of `readVec`, this requires pre-allocating a vector of size `n`. In `inputAll`, however, the size of the vector is not known _a priori_ and if the argument to `inputN` is large, the allocation of a much-too-large buffer is wasteful. Therefore, for large input operations, query the remaining size of the reader using `avail`, and try to read that much. But one should also keep things rounded to the nearest `chunkSize`.
> - The use of `avail` to try to do (large) read operations of just the right size will be inaccurate on translated readers. But this inaccuracy can be tolerated: if the translation is anything close to 1-1, `avail` will still provide a very good hint about the order-of-magnitude size of what remains to be read.
> - Similar suggestions apply to very large output operations. Small outputs go through a buffer; the buffer is written with `writeArr`. Very large outputs can be written directly from the argument string using `writeVec`.
> - A lazy functional input stream can (should) be implemented as a sequence of immutable (vector) buffers, each with a mutable **`ref`** to the next \`\`thing,'' which is either another buffer, the underlying reader, or an indication that the stream has been truncated.
> - The `input` function should return the largest sequence that is most convenient. Usually this means \`\`the remaining contents of the current buffer.''
> - To support non-blocking input, use `readVecNB` if it exists, otherwise do `canInput` followed (if appropriate) by `readVec`.
> - To support blocking input, use `readVec` if it exists, otherwise do `readVecNB` followed (if it would block) by `block`. and then another `readVecNB`.
> - To support lazy functional streams, `readArr` and `readArrNB` are not useful. If necessary, `readVec` should be synthesized from `readArr` and `readVecNB` from `readArrNB`.
> - `writeArr` should, if necessary, be synthesized from `writeVec` and vice versa. Similarly for `writeArrNB` and `writeVecNB`.
