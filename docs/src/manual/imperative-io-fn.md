# <span id="section:0"></span>The `ImperativeIO` functor

---

#### Synopsis

<span id="ImperativeIO:FCT:SPEC"></span>

```sml
functor ImperativeIO ( ... ) : IMPERATIVE_IO (* OPTIONAL *)
```

The optional `ImperativeIO` functor can be used to implement (derive) an imperative-style stream I/O facility in terms of a lazy functional stream I/O facility. In the imperative style, input and output operations do not return a new stream each time but cause side-effects on their arguments. Most functions can raise the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception for various reasons, including illegal or inconsistent parameters, IO failures, and attempts to do I/O on closed output streams.

The `ImperativeIO` functor is not often needed, as the required [`BinIO`](bin-io.md#BinIO:STR:SPEC) and [`TextIO`](text-io.md#TextIO:STR:SPEC) structures supply imperative-style I/O for most situations. It plays a useful role when the programmer needs to construct I/O facilities with element types other than [`char`](char.md#SIG:CHAR.char:TY:SPEC) or [`Word8.word`](word.md#SIG:WORD.word:TY:SPEC), or ones based on user-specified I/O primitives.

---

#### Functor argument interface

```sml
structure StreamIO : STREAM_IO
structure Vector : MONO_VECTOR
structure Array : MONO_ARRAY
sharing type StreamIO.elem = Vector.elem = Array.elem
sharing type StreamIO.vector = Vector.vector = Array.vector
```

#### Description

<span id="ARG:ImperativeIO.StreamIO:STR"></span>**`structure`**` StreamIO `**`:`**` `[`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC)  
The particular functional stream I/O facility from which this imperative I/O facility is derived. Most functions just call functions in `StreamIO` and do a little extra bookkeeping.

#### Examples

```repl
TextIO.inputLine TextIO.stdIn;;
```

#### See Also

> [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC), [`MONO_ARRAY`](mono-array.md#MONO_ARRAY:SIG:SPEC), [`MONO_VECTOR`](mono-vector.md#MONO_VECTOR:SIG:SPEC), [`PrimIO`](prim-io-fn.md#PrimIO:FCT:SPEC), [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC), [`StreamIO`](stream-io-fn.md#StreamIO:FCT:SPEC)
