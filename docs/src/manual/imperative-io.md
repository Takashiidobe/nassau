# <span id="section:0"></span>The `IMPERATIVE_IO` signature

---

#### Synopsis

<span id="IMPERATIVE_IO:SIG:SPEC"></span>

```sml
signature IMPERATIVE_IO
```

The `IMPERATIVE_IO` signature defines the interface of the _Imperative I/O_ layer in the I/O stack. This layer provides buffered I/O using mutable, redirectable streams.

---

#### Interface

<span id="SIG:IMPERATIVE_IO.vector:TY:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.elem:TY:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.instream:TY:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.outstream:TY:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.input:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.input1:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.inputN:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.inputAll:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.canInput:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.lookahead:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.closeIn:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.endOfStream:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.output:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.output1:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.flushOut:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.closeOut:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.mkInstream:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.getInstream:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.setInstream:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.mkOutstream:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.getOutstream:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.setOutstream:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.getPosOut:VAL:SPEC"></span>
<span id="SIG:IMPERATIVE_IO.setPosOut:VAL:SPEC"></span>

```sml
structure StreamIO : STREAM_IO
type vector = StreamIO.vector
type elem = StreamIO.elem
type instream
type outstream
val input : instream -> vector
val input1 : instream -> elem option
val inputN : instream * int -> vector
val inputAll : instream -> vector
val canInput : instream * int -> int option
val lookahead : instream -> elem option
val closeIn : instream -> unit
val endOfStream : instream -> bool
val output : outstream * vector -> unit
val output1 : outstream * elem -> unit
val flushOut : outstream -> unit
val closeOut : outstream -> unit
val mkInstream : StreamIO.instream -> instream
val getInstream : instream -> StreamIO.instream
val setInstream : instream * StreamIO.instream -> unit
val mkOutstream : StreamIO.outstream -> outstream
val getOutstream : outstream -> StreamIO.outstream
val setOutstream : outstream * StreamIO.outstream -> unit
val getPosOut : outstream -> StreamIO.out_pos
val setPosOut : outstream * StreamIO.out_pos -> unit
```

#### Description

<span id="SIG:IMPERATIVE_IO.StreamIO:STR"></span>**`structure`**` StreamIO `**`:`**` `[`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC)  
This substructure provides lower-level stream I/O, as defined by the [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) interface, which is compatible with the [`instream`](imperative-io.md#SIG:IMPERATIVE_IO.instream:TY:SPEC) and [`outstream`](imperative-io.md#SIG:IMPERATIVE_IO.outstream:TY:SPEC) types, in the sense that the conversion functions [`mkInstream`](imperative-io.md#SIG:IMPERATIVE_IO.mkInstream:VAL:SPEC), [`getInstream`](imperative-io.md#SIG:IMPERATIVE_IO.getInstream:VAL:SPEC), [`mkOutstream`](imperative-io.md#SIG:IMPERATIVE_IO.mkOutstream:VAL:SPEC), and [`getOutstream`](imperative-io.md#SIG:IMPERATIVE_IO.getOutstream:VAL:SPEC) allow the programmer to convert between low-level streams and redirectable streams. Typically, the redirectable streams are implemented in terms of low-level streams. Note that [`StreamIO.outstream`](stream-io.md#SIG:STREAM_IO.outstream:TY:SPEC) is not a functional stream. The _only_ difference between a [`StreamIO.outstream`](stream-io.md#SIG:STREAM_IO.outstream:TY:SPEC) and an [`outstream`](imperative-io.md#SIG:IMPERATIVE_IO.outstream:TY:SPEC) is that the latter can be redirected.

<span id="SIG:IMPERATIVE_IO.vector:TY"></span>**`type`**` vector = StreamIO.vector`
**`type`**` elem = StreamIO.elem`  
These are the abstract types of stream elements and vectors of elements. For text streams, these are [`Char.char`](char.md#SIG:CHAR.char:TY:SPEC) and [`String.string`](string.md#SIG:STRING.string:TY:SPEC), while for binary streams, they correspond to [`Word8.word`](word.md#SIG:WORD.word:TY:SPEC) and [`Word8Vector.vector`](mono-vector.md#SIG:MONO_VECTOR.vector:TY:SPEC).

<span id="SIG:IMPERATIVE_IO.instream:TY"></span>**`type`**` instream`  
The type of redirectable imperative input streams. Two imperative streams may share an underlying functional stream or reader. Closing one of them effectively closes the underlying functional stream, which will affect subsequent operations on the other.

<span id="SIG:IMPERATIVE_IO.outstream:TY"></span>**`type`**` outstream`  
The type of redirectable output streams. Two redirectable streams may share an underlying stream or writer. If this is the case, writing or positioning the file pointer on one of them, or closing it, also affects the other.

<span id="SIG:IMPERATIVE_IO.input:VAL"></span>
`input ``strm`` `  
attempts to read from `strm`, starting from the current input file position. When elements are available, it returns a `vector` of at least one element. When `strm` is at end-of-stream or is closed, it returns an empty vector. Otherwise, `input` blocks until one of these conditions is met, and returns accordingly. It may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC).

<span id="SIG:IMPERATIVE_IO.input1:VAL"></span>
`input1 ``strm`` `  
reads one element from `strm`. It returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(e)` if one element was available; it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if at end-of-stream. It may block, and may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC).

After a call to [`input1`](imperative-io.md#SIG:IMPERATIVE_IO.input1:VAL:SPEC) returning [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) to indicate an end-of-stream, the input stream should be positioned after the end-of-stream.

<span id="SIG:IMPERATIVE_IO.inputN:VAL"></span>
`inputN (``strm``, ``n``) `  
reads at most `n` elements from `strm`. It returns a vector containing `n` elements if at least `n` elements are available before end-of-stream; it returns a shorter (and possibly empty) vector of all elements remaining before end-of-stream otherwise. It may block, and may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC). It raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if `n` \< 0 or if `n` is greater than the [`maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC) value for the [`vector`](mono-vector.md#SIG:MONO_VECTOR.vector:TY:SPEC) type.

<span id="SIG:IMPERATIVE_IO.inputAll:VAL"></span>
`inputAll ``strm`` `  
returns all elements of `strm` up to end-of-stream. It may block, and may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC). It raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the amount of data exceeds the [`maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC) of the [`vector`](imperative-io.md#SIG:IMPERATIVE_IO.vector:TY:SPEC) type.

<span id="SIG:IMPERATIVE_IO.canInput:VAL"></span>
`canInput (``strm``, ``n``) `  
returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if any attempt at input would block. It returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``k``)`, where 0 \<= `k` \<= `n`, if a call to [`input`](imperative-io.md#SIG:IMPERATIVE_IO.input:VAL:SPEC) would return immediately with at least `k` characters. Note that `k` = 0 corresponds to the stream being at end-of-stream.

Some streams may not support this operation, in which case the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception will be raised. This function also raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying system calls. It raises the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception if `n` \< 0.

> **Implementation note:**
>
> It is suggested that implementations of [`canInput`](imperative-io.md#SIG:IMPERATIVE_IO.canInput:VAL:SPEC) should attempt to return as large a `k` as possible. For example, if the buffer contains 10 characters and the user calls `canInput (``f``, 15)`, [`canInput`](imperative-io.md#SIG:IMPERATIVE_IO.canInput:VAL:SPEC) should call `readVecNB(5)` to see if an additional 5 characters are available.


<span id="SIG:IMPERATIVE_IO.lookahead:VAL"></span>
`lookahead ``strm`` `  
determines whether one element is available on `strm` before end-of-stream and returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(e)` in this case; it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if at end-of-stream. In the former case, `e` is not removed from `strm` but stays available for further input operations. It may block, and may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC).

The underlying [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) stream can be used to easily implement arbitrary lookahead.

<span id="SIG:IMPERATIVE_IO.closeIn:VAL"></span>
`closeIn ``strm`` `  
closes the input stream `strm`, freeing resources of the underlying I/O layers associated with it. Closing an already closed stream will be ignored. Other operations on a closed stream will behave as if the stream is at end-of-stream. The function is implemented in terms of [`StreamIO.closeIn`](stream-io.md#SIG:STREAM_IO.closeIn:VAL:SPEC). It may also raise [`Io`](io.md#SIG:IO.Io:EXN:SPEC) when another error occurs.

<span id="SIG:IMPERATIVE_IO.endOfStream:VAL"></span>
`endOfStream ``strm`` `  
returns `true` if `strm` is at end-of-stream, and `false` if elements are still available. It may block until one of these conditions is determined, and may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC).

When `endOfStream` returns `true` on an untruncated stream, this denotes the _current_ situation. After a read from `strm` to consume the end-of-stream, it is possible that the next call to `endOfStream ``strm` may return false, and input operations will deliver new elements. For further information, consult the description of [`STREAM_IO.endOfStream`](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC).

<span id="SIG:IMPERATIVE_IO.output:VAL"></span>
`output (``strm``, ``vec``) `  
attempts to write the contents of `vec` to `strm`, starting from the current output file position. It may block until the underlying layers (and eventually the operating system) can accept all of `vec`. It may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC). In that case, it is unspecified how much of `vec` was actually written.

<span id="SIG:IMPERATIVE_IO.output1:VAL"></span>
`output1 (``strm``, ``el``) `  
writes exactly one element `el` to `strm`. It may block, and may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if an error occurs. In that case, it is unspecified how much of `el` was actually written, especially if its physical representation is larger than just one byte. At this level, more than this cannot be guaranteed. Programs that need more control over this possibility need to make use of more primitive or OS-specific I/O routines.

<span id="SIG:IMPERATIVE_IO.flushOut:VAL"></span>
`flushOut ``strm`` `  
causes any buffers associated with `strm` to be written out. It is implemented in terms of [`StreamIO.flushOut`](stream-io.md#SIG:STREAM_IO.flushOut:VAL:SPEC). The function may block, and may raise the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) when an error occurs.

<span id="SIG:IMPERATIVE_IO.closeOut:VAL"></span>
`closeOut ``strm`` `  
flushes any buffers associated with `strm`, then closes `strm`, freeing resources of the underlying I/O layers associated with it. It is implemented in terms of [`StreamIO.closeOut`](stream-io.md#SIG:STREAM_IO.closeOut:VAL:SPEC). A write attempt on a closed [`outstream`](imperative-io.md#SIG:IMPERATIVE_IO.outstream:TY:SPEC) will cause the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC)`{cause=`[`ClosedStream`](io.md#SIG:IO.ClosedStream:EXN:SPEC)`,...}` to be raised. It may also raise [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if another error occurs (_e.g._, buffers cannot be flushed out).

<span id="SIG:IMPERATIVE_IO.mkInstream:VAL"></span>
`mkInstream ``strm`` `  
constructs a redirectable input stream from a functional one. The current version of `strm` returned by input operations will be kept internally and used for the next input. They can be obtained by `getInstream`.

<span id="SIG:IMPERATIVE_IO.getInstream:VAL"></span>
`getInstream ``strm`` `  
returns the current version of the underlying functional input stream of `strm`. Using [`getInstream`](imperative-io.md#SIG:IMPERATIVE_IO.getInstream:VAL:SPEC), it is possible to get input directly from the underlying functional stream. After having done so, it may be necessary to reassign the newly obtained functional stream to `strm` using [`setInstream`](imperative-io.md#SIG:IMPERATIVE_IO.setInstream:VAL:SPEC); otherwise the previous input will be read again when reading from `strm` the next time.

<span id="SIG:IMPERATIVE_IO.setInstream:VAL"></span>
`setInstream (``strm``, ``strm'``) `  
assigns a new functional stream `strm'` to `strm`. Future input on `strm` will be read from `strm'`. This is useful for redirecting input or interleaving input from different streams, _e.g._, when handling nested include files in a lexer.

<span id="SIG:IMPERATIVE_IO.mkOutstream:VAL"></span>
`mkOutstream ``strm`` `  
constructs a redirectable output stream from a low-level functional one. Output to the imperative stream will be redirected to `strm`.

<span id="SIG:IMPERATIVE_IO.getOutstream:VAL"></span>
`getOutstream ``strm`` `  
flushes `strm` and returns the underlying [`StreamIO`](imperative-io.md#SIG:IMPERATIVE_IO.StreamIO:STR:SPEC) output stream. Using [`getOutstream`](imperative-io.md#SIG:IMPERATIVE_IO.getOutstream:VAL:SPEC), it is possible to write output directly to the underlying stream, or to save it and restore it using [`setOutstream`](imperative-io.md#SIG:IMPERATIVE_IO.setOutstream:VAL:SPEC) after `strm` has been redirected.

<span id="SIG:IMPERATIVE_IO.setOutstream:VAL"></span>
`setOutstream (``strm``, ``strm'``) `  
flushes the stream underlying `strm`, and then assigns a new low-level stream `strm'` to it. Future output on `strm` will be redirected to `strm'`.

<span id="SIG:IMPERATIVE_IO.getPosOut:VAL"></span>
`getPosOut ``strm`` `  
returns the current position in the stream `strm`. This raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if the stream does not support the operation, among other reasons. See [`StreamIO.getPosOut`](stream-io.md#SIG:STREAM_IO.getPosOut:VAL:SPEC).

<span id="SIG:IMPERATIVE_IO.setPosOut:VAL"></span>
`setPosOut (``strm``, ``pos``) `  
sets the current position of the stream `strm` to be `pos`. This raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if the stream does not support the operation, among other reasons. See [`StreamIO.setPosOut`](stream-io.md#SIG:STREAM_IO.setPosOut:VAL:SPEC).

#### Examples

```repl
TextIO.inputLine TextIO.stdIn;;
```

#### See Also

> [`BinIO`](bin-io.md#BinIO:STR:SPEC), [`ImperativeIO`](imperative-io-fn.md#ImperativeIO:FCT:SPEC), [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC), [`TextIO`](text-io.md#TextIO:STR:SPEC)

#### Discussion

A word is in order concerning I/O nomenclature. We refer to the I/O provided by the [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC) signature as Imperative I/O, while the I/O provided by the [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) signature is called Stream I/O. On the other hand, the type of buffered I/O handled by both of these layers is typically considered \`\`stream I/O,'' which explains why the I/O objects defined in both levels are called `instream` and `outstream`. To avoid confusion, we sometimes refer to I/O using the Stream I/O layer as functional, focusing on the functional flavor of the input streams at that level. This, however, glosses over the imperative nature of output at the same level. The principal distinction between the two layers is that I/O using [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC) can be redirected, while I/O using [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) cannot.

The semantics of Imperative I/O operations are (almost) all defined in terms of the operations provided by the underlying [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) substructure. Specifically, we have the reference implementations:

fun input(f) = let 
      val (s,g) = StreamIO.input(getInstream f)
      in setInstream(f,g); s
      end
fun inputAll(f)= let 
      val (s,g) = StreamIO.inputAll(getInstream f)
      in setInstream(f,g); s
      end
fun endOfStream(f)= StreamIO.endOfStream(getInstream f)
fun output(f,s) = StreamIO.output(getOutStream f, s)
fun flushOut(f) = StreamIO.flushOut(getOutStream f)

with similar implementations for other imperative I/O operations.

Alternatively, we can consider Imperative I/O streams as [`ref`]() cells referring to [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) streams:

type instream = StreamIO.instream ref
type outstream = StreamIO.outstream ref
fun input strm = let 
      val (v, strm') = StreamIO.input(!strm)
      in
        strm := strm'; v
      end
fun output (strm, v) = StreamIO.output(!strm, v)

etc.

The one exception to the above approaches is [`input1`](imperative-io.md#SIG:IMPERATIVE_IO.input1:VAL:SPEC). If an implementation relies solely on [`StreamIO.input1`](stream-io.md#SIG:STREAM_IO.input1:VAL:SPEC), [`input1`](imperative-io.md#SIG:IMPERATIVE_IO.input1:VAL:SPEC) could never advance beyond an end-of-stream. To avoid this, a reference implementation for [`input1`](imperative-io.md#SIG:IMPERATIVE_IO.input1:VAL:SPEC) would be:

fun input1 f = let
      val (s,g) = StreamIO.inputN(getInstream f, 1)
      in setInstream(f,g);
         if length s = 0 then NONE
         else SOME(sub(s,0))
      end

Limited random access on input streams --- that is, returning to a previously scanned position --- can be accomplished using [`getInstream`](imperative-io.md#SIG:IMPERATIVE_IO.getInstream:VAL:SPEC) and the underlying Stream I/O layer:

fun reread (f : instream, n : int) = let
      val g = getInstream(f)
      val s = inputN (f,n)
      in
        setInstream(f,g);
        (s, inputN (f,n))
      end

The pair of vectors returned by `reread` will always be identical. Similarly limited random access on output streams can be done directly using [`getPosOut`](imperative-io.md#SIG:IMPERATIVE_IO.getPosOut:VAL:SPEC) and [`setPosOut`](imperative-io.md#SIG:IMPERATIVE_IO.setPosOut:VAL:SPEC). More general random access is only available at the Primitive I/O level.

> **Implementation note:**
>
> Input on a closed stream behaves as though the stream is permanently at end-of-stream. Thus, in addition to closing the underlying functional stream, the [`closeIn`](imperative-io.md#SIG:IMPERATIVE_IO.closeIn:VAL:SPEC) function must also replace the functional stream with an empty stream.
