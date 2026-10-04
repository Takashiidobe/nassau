# <span id="section:0"></span>The `STREAM_IO` signature

---

#### Synopsis

<span id="STREAM_IO:SIG:SPEC"></span>

```sml
signature STREAM_IO
```

The `STREAM_IO` signature defines the interface of the _Stream I/O_ layer in the I/O stack. This layer provides buffering over the readers and writers of the [Primitive I/O](prim-io.md#PrimIO:Layer) layer.

Input streams are treated in the lazy functional style: that is, input from a stream `f` yields a finite vector of elements, plus a new stream `f'`. Input from `f` again will yield the same elements; to advance within the stream in the usual way, it is necessary to do further input from `f'`. This interface allows arbitrary lookahead to be done very cleanly, which should be useful both for _ad hoc_ lexical analysis and for table-driven, regular-expression-based lexing.

Output streams are handled more conventionally, since the lazy functional style does not seem to make sense for output.

Stream I/O functions may raise the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception if a resulting vector of elements would exceed the maximum vector size, or the [`IO.Io`](io.md#SIG:IO.Io:EXN:SPEC) exception. In general, when [`IO.Io`](io.md#SIG:IO.Io:EXN:SPEC) is raised as a result of a failure in a lower-level module, the underlying exception is caught and propagated up as the `cause` component of the [`IO.Io`](io.md#SIG:IO.Io:EXN:SPEC) exception value. This will usually be a [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC), [`IO.ClosedStream`](io.md#SIG:IO.ClosedStream:EXN:SPEC), [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC), or [`Fail`](general.md#SIG:GENERAL.Fail:EXN:SPEC) exception (the last possible because of user-supplied readers or writers), but the stream I/O module will rarely (perhaps never) need to inspect it.

---

#### Interface

<span id="SIG:STREAM_IO.elem:TY:SPEC"></span>
<span id="SIG:STREAM_IO.vector:TY:SPEC"></span>
<span id="SIG:STREAM_IO.instream:TY:SPEC"></span>
<span id="SIG:STREAM_IO.outstream:TY:SPEC"></span>
<span id="SIG:STREAM_IO.out_pos:TY:SPEC"></span>
<span id="SIG:STREAM_IO.reader:TY:SPEC"></span>
<span id="SIG:STREAM_IO.writer:TY:SPEC"></span>
<span id="SIG:STREAM_IO.pos:TY:SPEC"></span>
<span id="SIG:STREAM_IO.input:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.input1:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.inputN:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.inputAll:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.canInput:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.closeIn:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.endOfStream:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.output:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.output1:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.flushOut:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.closeOut:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.mkInstream:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.getReader:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.filePosIn:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.setBufferMode:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.getBufferMode:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.mkOutstream:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.getWriter:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.getPosOut:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.setPosOut:VAL:SPEC"></span>
<span id="SIG:STREAM_IO.filePosOut:VAL:SPEC"></span>

```sml
type elem
type vector
type instream
type outstream
type out_pos
type reader
type writer
type pos
val input : instream -> vector * instream
val input1 : instream -> (elem * instream) option
val inputN : instream * int -> vector * instream
val inputAll : instream -> vector * instream
val canInput : instream * int -> int option
val closeIn : instream -> unit
val endOfStream : instream -> bool
val output : outstream * vector -> unit
val output1 : outstream * elem -> unit
val flushOut : outstream -> unit
val closeOut : outstream -> unit
val mkInstream : reader * vector -> instream
val getReader : instream -> reader * vector
val filePosIn : instream -> pos
val setBufferMode : outstream * IO.buffer_mode -> unit
val getBufferMode : outstream -> IO.buffer_mode
val mkOutstream : writer * IO.buffer_mode -> outstream
val getWriter : outstream -> writer * IO.buffer_mode
val getPosOut : outstream -> out_pos
val setPosOut : out_pos -> outstream
val filePosOut : out_pos -> pos
```

#### Description

<span id="SIG:STREAM_IO.elem:TY"></span>**`type`**` elem`
**`type`**` vector`  
The abstract types of stream elements and vectors of elements. For text streams, these are [`Char.char`](char.md#SIG:CHAR.char:TY:SPEC) and [`String.string`](string.md#SIG:STRING.string:TY:SPEC), while for binary streams, these are [`Word8.word`](word.md#SIG:WORD.word:TY:SPEC) and [`Word8Vector.vector`](mono-vector.md#SIG:MONO_VECTOR.vector:TY:SPEC).

<span id="SIG:STREAM_IO.instream:TY"></span>**`type`**` instream`  
The type of buffered functional input streams.

Input streams are in one of three states: active, truncated, or closed. When initially created, the stream is active. When disconnected from its underlying primitive reader (_e.g._, by [`getReader`](stream-io.md#SIG:STREAM_IO.getReader:VAL:SPEC)), the stream is truncated. When [`closeIn`](stream-io.md#SIG:STREAM_IO.closeIn:VAL:SPEC) is applied to the stream, the stream enters the closed state. A closed stream is also truncated. The only real difference between a truncated stream and a closed one is that in the latter case, the stream's primitive I/O reader is closed.

Reading from a truncated input stream will never block; after all buffered elements are read, input operations always return empty vectors.

<span id="SIG:STREAM_IO.outstream:TY"></span>**`type`**` outstream`  
The type of buffered output streams. Unlike input streams, these are imperative objects.

Output streams are in one of three states: active, terminated, or closed. When initially created, the stream is active. When disconnected from its underlying primitive writer (_e.g._, by [`getWriter`](stream-io.md#SIG:STREAM_IO.getWriter:VAL:SPEC)), the stream is terminated. When [`closeOut`](stream-io.md#SIG:STREAM_IO.closeOut:VAL:SPEC) is applied to the stream, the stream enters the closed state. A closed stream is also terminated. The only real difference between a terminated stream and a closed one is that in the latter case, the stream's primitive I/O writer is closed.

In a terminated output stream, there is no mechanism for performing more output, so any output operations will raise the [`IO.Io`](io.md#SIG:IO.Io:EXN:SPEC) exception.

<span id="SIG:STREAM_IO.out_pos:TY"></span>**`type`**` out_pos`  
The type of positions in output streams. This can be used to reconstruct an output stream at the position recorded in the [`out_pos`](stream-io.md#SIG:STREAM_IO.out_pos:TY:SPEC) value. Thus, the canonical representation for the type is `(`[`outstream`](stream-io.md#SIG:STREAM_IO.outstream:TY:SPEC)`*`[`pos`](stream-io.md#SIG:STREAM_IO.pos:TY:SPEC)`)`.

<span id="SIG:STREAM_IO.reader:TY"></span>**`type`**` reader`
**`type`**` writer`  
The types of the readers and writers that underlie the input and output streams.

<span id="SIG:STREAM_IO.pos:TY"></span>**`type`**` pos`  
This is the type of positions in the underlying readers and writers. In some instantiations of this signature (_e.g._, [`TextIO.StreamIO`](text-io.md#SIG:TEXT_IO.StreamIO:STR:SPEC)), [`pos`](stream-io.md#SIG:STREAM_IO.pos:TY:SPEC) is abstract; in others it may be concrete (_e.g._, [`Position.int`](integer.md#SIG:INTEGER.int:TY:SPEC) in [`BinIO.StreamIO`](imperative-io.md#SIG:IMPERATIVE_IO.StreamIO:STR:SPEC)).

<span id="SIG:STREAM_IO.input:VAL"></span>
`input ``f`` `  
returns a vector of one or more elements from `f` and the remainder of the stream, if any elements are available. If an end-of-stream has been reached, then the empty vector is returned. The function may block until one of these conditions is satisfied. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying reader.

<span id="SIG:STREAM_IO.input1:VAL"></span>
`input1 ``f`` `  
returns the next element in the stream `f` and the remainder of the stream. If the stream is at the end, then [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) is returned. It may block until one of these conditions is satisfied. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying reader.

<span id="SIG:STREAM_IO.inputN:VAL"></span>
`inputN (``f``, ``n``) `  
returns a vector of the next `n` elements from `f` and the rest of the stream. If fewer than `n` elements are available before the next end-of-stream, it returns all of the elements up to that end-of-stream. It may block until it can determine if additional characters are available or an end-of-stream condition holds. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying reader. It raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if `n` \< 0 or the number of elements to be returned is greater than [`maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC). Also, [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC)`(f,0)` returns immediately with an empty vector and `f`, so this cannot be used as an indication of end-of-stream.

Using [`instream`](stream-io.md#SIG:STREAM_IO.instream:TY:SPEC)s, one can synthesize a non-blocking version of [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC) from [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC) and [`canInput`](stream-io.md#SIG:STREAM_IO.canInput:VAL:SPEC), as [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC) is guaranteed not to block if a previous call to [`canInput`](stream-io.md#SIG:STREAM_IO.canInput:VAL:SPEC) returned [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(_)`.

<span id="SIG:STREAM_IO.inputAll:VAL"></span>
`inputAll ``f`` `  
returns the vector of the rest of the elements in the stream `f` (_i.e._, up to an end-of-stream), and a new stream `f'`. Care should be taken when using this function, since it can block indefinitely on interactive streams. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying reader. The stream `f'` is immediately past the next end-of-stream of `f`. For ordinary files in which only one end-of stream is expected, `f'` can be ignored. If a file has multiple end-of-stream conditions (which can happen under some operating systems), [`inputAll`](stream-io.md#SIG:STREAM_IO.inputAll:VAL:SPEC) returns all the elements up to the next end-of-stream. It raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the number of elements to be returned is greater than [`maxLen`](mono-vector.md#SIG:MONO_VECTOR.maxLen:VAL:SPEC) for the relevant [`vector`](stream-io.md#SIG:STREAM_IO.vector:TY:SPEC) type.

<span id="SIG:STREAM_IO.canInput:VAL"></span>
`canInput (``f``, ``n``) `  
returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC) if any attempt at input would block. It returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``k``)`, where 0 \<= `k` \<= `n`, if a call to [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) would return immediately with at least `k` characters. Note that `k` = 0 corresponds to the stream being at end-of-stream.

Some streams may not support this operation, in which case the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception will be raised. This function also raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying reader. It raises the [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) exception if `n` \< 0.

> **Implementation note:**
>
> It is suggested that implementations of [`canInput`](stream-io.md#SIG:STREAM_IO.canInput:VAL:SPEC) should attempt to return as large a `k` as possible. For example, if the buffer contains 10 characters and the user calls `canInput (``f``, 15)`, [`canInput`](stream-io.md#SIG:STREAM_IO.canInput:VAL:SPEC) should call `readVecNB(5)` to see if an additional 5 characters are available.
>
> Such a lookahead commits the stream to the characters read by `readVecNB` but it does not commit the stream to return those characters on the next call to [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC). Indeed, a typical implementation will simply return the remainder of the current buffer, in this case, consisting of 10 characters, if [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) is called. On the other hand, an implementation can decide to always respond to [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) with all the elements currently available, provided an earlier call to [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) has not committed the stream to a particular response. The only requirement is that any future call of [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) on the same input stream must return the same vector of elements.


<span id="SIG:STREAM_IO.closeIn:VAL"></span>
`closeIn ``f`` `  
marks the stream closed, and closes the underlying reader. Applying [`closeIn`](stream-io.md#SIG:STREAM_IO.closeIn:VAL:SPEC) on a closed stream has no effect. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying reader.

<span id="SIG:STREAM_IO.endOfStream:VAL"></span>
`endOfStream ``f`` `  
tests if `f` satisfies the end-of-stream condition. If there is no further input in the stream, then this returns `true`; otherwise it returns `false`. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying reader.

This function may block when checking for more input. It is equivalent to

([length](mono-vector.md#SIG:MONO_VECTOR.length:VAL:SPEC)(#1([input](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) `f`)) = 0)

where [`length`](mono-vector.md#SIG:MONO_VECTOR.length:VAL:SPEC) is the vector length operation

Note that even if [`endOfStream`](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC) returns `true`, subsequent input operations may succeed if more data becomes available. A stream can have multiple end-of-streams interspersed with normal elements. This can happen on Unix, for example, if a user types control-D (`#"\^D"`) on a terminal device, and then keeps typing characters; it may also occur on file descriptors connected to sockets.

Multiple end-of-streams is a property of the underlying reader. Thus, `readVec` on a [`reader`](stream-io.md#SIG:STREAM_IO.reader:TY:SPEC) may return an empty string, then another call to `readVec` on the same [`reader`](stream-io.md#SIG:STREAM_IO.reader:TY:SPEC) may return a nonempty string, then a third call may return an empty string. It is always true, however, that

[endOfStream](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC) `f` = [endOfStream](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC) `f`

In addition, if [`endOfStream`](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC)` ``f` returns `true`, then [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC)` ``f` returns `("",``f'``)` and [`endOfStream`](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC)` ``f'` may or may not be true.

<span id="SIG:STREAM_IO.output:VAL"></span>
`output (``f``, ``vec``) `  
writes the vector of elements `vec` to the stream `f`. This raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if `f` is terminated. This function also raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying writer.

<span id="SIG:STREAM_IO.output1:VAL"></span>
`output1 (``f``, ``el``) `  
writes the element `el` to the stream `f`. This raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if `f` is terminated. This function also raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying writer.

<span id="SIG:STREAM_IO.flushOut:VAL"></span>
`flushOut ``f`` `  
flushes any output in `f`'s buffer to the underlying writer; it is a no-op on terminated streams. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying writer.

<span id="SIG:STREAM_IO.closeOut:VAL"></span>
`closeOut ``f`` `  
flushes `f`'s buffers, marks the stream closed, and closes the underlying writer. This operation has no effect if `f` is already closed. Note that if `f` is terminated, no flushing will occur. This function raises the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if there is an error in the underlying writer or if flushing fails. In the latter case, the stream is left open.

<span id="SIG:STREAM_IO.mkInstream:VAL"></span>
`mkInstream (``rd``, ``v``) `  
returns a new [`instream`](stream-io.md#SIG:STREAM_IO.instream:TY:SPEC) built on top of the reader `rd` with the initial buffer contents `v`.

If the reader does not implement all of its fields (for example, if random access operations are missing), then certain operations will raise exceptions when applied to the resulting [`instream`](stream-io.md#SIG:STREAM_IO.instream:TY:SPEC). The following table describes the minimal relationship between [`instream`](stream-io.md#SIG:STREAM_IO.instream:TY:SPEC) operations and a reader:

---

**instream supports:**

**if reader implements:**

[`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC), [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC), etc.

`readVec`

[`canInput`](stream-io.md#SIG:STREAM_IO.canInput:VAL:SPEC)

`readVecNB`

[`endOfStream`](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC)

`readVec`

[`filePosIn`](stream-io.md#SIG:STREAM_IO.filePosIn:VAL:SPEC)

`getPos` and `setPos`

---

If the reader provides more operations, the resulting stream may use them.

[`mkInstream`](stream-io.md#SIG:STREAM_IO.mkInstream:VAL:SPEC) should construct the input stream using the reader provided. If the user wishes to employ synthesized functions in the reader, the user may call [`mkInstream`](stream-io.md#SIG:STREAM_IO.mkInstream:VAL:SPEC) with an augmented reader [`augmentReader`](prim-io.md#SIG:PRIM_IO.augmentReader:VAL:SPEC)`(``rd``)`. See [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC) for a description of the functions generated by [`augmentReader`](prim-io.md#SIG:PRIM_IO.augmentReader:VAL:SPEC).

Building more than one input stream on top of a single reader has unpredictable effects, since readers are imperative objects. In general, there should be a 1-1 correspondence between a reader and a sequence of input streams. Also note that creating an input stream this way means that the stream could be unaware that the reader has been closed until the stream actually attempts to read from it.

<span id="SIG:STREAM_IO.getReader:VAL"></span>
`getReader ``f`` `  
marks the input stream `f` as truncated and returns the underlying reader along with any unconsumed data from its buffer. The data returned will have the value `(closeIn f; inputAll f)`. The function raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if `f` is closed or truncated.

<span id="SIG:STREAM_IO.filePosIn:VAL"></span>
`filePosIn ``f`` `  
returns the primitive-level reader position that corresponds to the next element to be read from the buffered stream `f`. This raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if the stream does not support the operation, or if `f` has been truncated.

It should be true that, if `#1(inputAll ``f``)` returns vector `v`, then

(setPos (filePosIn `f`); readVec (length v))

should also return `v`, assuming all operations are defined and terminate.

> **Implementation note:**
>
> If the [`pos`](stream-io.md#SIG:STREAM_IO.pos:TY:SPEC) type is a concrete integer corresponding to a byte offset, and the translation function (between bytes and elements) is known, the value can be computed directly. If not, the value is given by
>
> fun pos (bufp, n, r as RD rdr) = let
>       val readVec = valOf (#readVec rdr)
>       val getPos = valOf (#getPos rdr)
>       val setPos = valOf (#setPos rdr)
>       val savep = getPos()
>       in
>         setPos bufp;
>         readVec n;
>         getPos () before setPos savep
>       end
>
> where `bufp` is the file position corresponding to the beginning of the current buffer, `n` is the number of elements already read from the current buffer, and `r` is the stream's underlying reader.


<span id="SIG:STREAM_IO.setBufferMode:VAL"></span>
`setBufferMode (``f``, ``mode``) `
` getBufferMode ``f`` `  
These functions set and get the buffering mode of the output stream `f`. Setting the buffer mode to [`IO.NO_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC) causes any buffered output to be flushed. If the flushing fails, the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception is raised. Switching the mode between [`IO.LINE_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC) and [`IO.BLOCK_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC) should not cause flushing. If, in going from [`IO.BLOCK_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC) to [`IO.LINE_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC), the user desires that the buffer contain no newline characters, the user should call [`flushOut`](stream-io.md#SIG:STREAM_IO.flushOut:VAL:SPEC) explicitly.

<span id="SIG:STREAM_IO.mkOutstream:VAL"></span>
`mkOutstream (``wr``, ``mode``) `  
returns a new output stream built on top of the writer `wr` with the indicated buffer mode.

If the writer does not implement all of its fields (for example, if random access operations are missing), then certain operations will raise exceptions when applied to the resulting [`outstream`](stream-io.md#SIG:STREAM_IO.outstream:TY:SPEC). The following table describes the minimal relationship between [`outstream`](stream-io.md#SIG:STREAM_IO.outstream:TY:SPEC) operations and a writer:

---

**outstream supports:**

**if augmented writer implements:**

[`output`](stream-io.md#SIG:STREAM_IO.output:VAL:SPEC), [`output1`](stream-io.md#SIG:STREAM_IO.output1:VAL:SPEC), etc.

`writeArr`

[`flushOut`](stream-io.md#SIG:STREAM_IO.flushOut:VAL:SPEC)

`writeArr`

[`setBufferMode`](stream-io.md#SIG:STREAM_IO.setBufferMode:VAL:SPEC)

`writeArr`

[`getPosOut`](stream-io.md#SIG:STREAM_IO.getPosOut:VAL:SPEC)

`writeArr` and `getPos`

[`setPosOut`](stream-io.md#SIG:STREAM_IO.setPosOut:VAL:SPEC)

`writeArr` and `setPos`

---

If the writer provides more operations, the resulting stream may use them.

[`mkOutstream`](stream-io.md#SIG:STREAM_IO.mkOutstream:VAL:SPEC) should construct the output stream using the writer provided. If the user wishes to employ synthesized functions in the writer, the user may call [`mkOutstream`](stream-io.md#SIG:STREAM_IO.mkOutstream:VAL:SPEC) with an augmented writer [`augmentWriter`](prim-io.md#SIG:PRIM_IO.augmentWriter:VAL:SPEC)`(``wr``)`. See [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC) for a description of the functions generated by [`augmentWriter`](prim-io.md#SIG:PRIM_IO.augmentWriter:VAL:SPEC).

Building more than one [`outstream`](stream-io.md#SIG:STREAM_IO.outstream:TY:SPEC) on top of a single writer has unpredictable effects, since buffering may change the order of output. In general, there should be a 1-1 correspondence between a writer and an output stream. Also note that creating an output stream this way means that the stream could be unaware that the writer has been closed until the stream actually attempts to write to it.

<span id="SIG:STREAM_IO.getWriter:VAL"></span>
`getWriter ``f`` `  
flushes the stream `f`, marks it as being terminated and returns the underlying writer and the stream's buffer mode. This raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if `f` is closed, or if the flushing fails.

<span id="SIG:STREAM_IO.getPosOut:VAL"></span>
`getPosOut ``f`` `  
returns the current position of the stream `f`. This raises the exception [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if the stream does not support the operation, if any implicit flushing fails, or if `f` is terminated.

> **Implementation note:**
>
> A typical implementation of this function will require calculating a value of type [`pos`](stream-io.md#SIG:STREAM_IO.pos:TY:SPEC), capturing where the next element written to `f` will be written in the underlying file. If the [`pos`](stream-io.md#SIG:STREAM_IO.pos:TY:SPEC) type is a concrete integer corresponding to a byte offset, and the translation function (between bytes and elements) is known, the value can be computed directly using `getPos`. If not, the value is given by
>
> fun pos (f, w as WR wtr) = let
>       val getPos = valOf (#getPos wtr)
>       in
>         flushOut f;
>         getPos ()
>       end
>
> where `f` is the output stream and `w` is the stream's underlying writer.


<span id="SIG:STREAM_IO.setPosOut:VAL"></span>
`setPosOut ``opos`` `  
flushes the output buffer of the stream underlying `opos`, sets the current position of the stream to the position recorded in `opos`, and returns the stream. This can raise an [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception if the flushing fails, if the stream does not support the operation, or if the stream underlying `opos` is terminated.

<span id="SIG:STREAM_IO.filePosOut:VAL"></span>
`filePosOut ``opos`` `  
returns the primitive-level writer position that corresponds to the abstract output stream position `opos`.

Suppose we are given an output stream `f` and a vector of elements `v`, and let `opos` equal `getPosOut(f)`. Then the code

(setPos opos; writeVec{buf=v,i=0,sz=NONE})

should have the same effect as the last line of the function

fun put (outs,x) = (flushOut outs;
                    output(outs,x);flushOut outs)

when called with `(f,v)` assuming all operations are defined and terminate, and that the call to `writeVec` returns `length v`.

#### Examples

```repl
TextIO.stdIn;;
```

#### See Also

> [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC), [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC), [`StreamIO`](stream-io-fn.md#StreamIO:FCT:SPEC), [`TEXT_STREAM_IO`](text-stream-io.md#TEXT_STREAM_IO:SIG:SPEC)

#### Discussion

The signature of [`input1`](stream-io.md#SIG:STREAM_IO.input1:VAL:SPEC) is inconsistent with all of the other input functions. This is intentional: its type makes it a `(char,instream) `[`StringCvt.reader`](string-cvt.md#SIG:STRING_CVT.reader:TY:SPEC) and thus a source of characters for the various scan functions.

Another point to notice about [`input1`](stream-io.md#SIG:STREAM_IO.input1:VAL:SPEC) is that it cannot be used to read beyond an end-of-stream. When an end-of-stream is encountered. the programmer will need to use one of the other input functions to obtain the stream after the end-of-stream. For example, if `input1(f)` returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC), a call to `inputN(f,1)` will return immediately with `(fromList [], f')`, and `f'` can be used to continue input.

It is possible that a stream's underlying reader/writer, or its operating system file descriptor, could be closed while the stream is still active. When this condition is detected, typically by an exception being raised by the lower level, the stream should raise the [`IO.Io`](io.md#SIG:IO.Io:EXN:SPEC) exception with `cause` set to [`IO.ClosedStream`](io.md#SIG:IO.ClosedStream:EXN:SPEC). On a related point, one can close a truncated or terminated string. This is intended as a convenience, with the inactive stream providing a handle to the underlying file, but it also provides an opportunity to close a reader or writer being actively used by another stream.

Output flushing can occur by calls to any output operation, or by calls to [`flushOut`](stream-io.md#SIG:STREAM_IO.flushOut:VAL:SPEC), [`closeOut`](stream-io.md#SIG:STREAM_IO.closeOut:VAL:SPEC), [`getWriter`](stream-io.md#SIG:STREAM_IO.getWriter:VAL:SPEC), [`setPosOut`](stream-io.md#SIG:STREAM_IO.setPosOut:VAL:SPEC), [`getPosOut`](stream-io.md#SIG:STREAM_IO.getPosOut:VAL:SPEC), or if [`setBufferMode`](stream-io.md#SIG:STREAM_IO.setBufferMode:VAL:SPEC) is called with mode [`IO.NO_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC). If flushing finds that it can do only a partial write (_i.e._, `writeVec` or a similar function returns a number of elements written less than its `sz` argument), then the stream function must adjust the stream's buffer for the items written and then try again. If the first or any successive write attempt raises an exception, then the stream function must raise the [`IO.Io`](io.md#SIG:IO.Io:EXN:SPEC) exception.

For the remainder of this chapter, we shall assume the following binding:

structure TS = TextIO.StreamIO

and that `elem = char`. Also, the predicates used to illustrate a point should all evaluate to true, assuming they complete without exception.

Input is semi-deterministic: [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) may read any number of elements from `f` the \`\`first'' time, but then it is committed to its choice, and must return the same number of elements on subsequent reads from the same point:

fun chkInput f= let 
      val (a,\_) = TS.input f
      val (b,\_) = TS.input f
      in a=b end

always returns true. In general, any expression involving input streams and functions defined in the [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) signature should always evaluate to the same value, barring exceptions.

Closing or truncating a stream just causes the not-yet-determined part of the stream to be empty:

fun chkClose f = let 
      val (a,f') = TS.input f
      val _ = TS.closeIn f
      val (b,\_) = TS.input f
      in  a=b andalso TS.endOfStream f' end

Closing a closed stream is legal and harmless:

fun closeTwice f = (TS.closeIn f; TS.closeIn f; true)

If a stream has already been at least partly determined, then [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) cannot possibly block:

fun noBlock f = let 
      val (s,\_) = TS.input f
      in 
        case TS.canInput (f, 1) of
          SOME 0 =\> (size s) = 0
        \| SOME _ =\> (size s) \> 0
        \| NONE =\> false
      end

Note that a successful [`canInput`](stream-io.md#SIG:STREAM_IO.canInput:VAL:SPEC) does not imply that more characters remain before end-of-stream, just that reading will not block.

A freshly opened stream is still undetermined (no \`\`read'' has yet been done on the underlying reader):

fun newStr rdr = let 
      val a = TS.mkInstream (rdr, "")
      in 
        TS.closeIn a;
        size(#1(TS.input a)) = 0
      end

This has the useful consequence that if one opens a stream, then extracts the underlying reader, the reader has not yet been advanced in its file.

A generalization of this property says that the first time any stream value is produced, it is up-to-date with respect to its reader:

fun nreads(f,0) = f 
  \| nreads(f,n) = let 
      val (\_,f') = TS.input f
      in 
        nreads(f',n-1)
      end
fun reads (rdr, n) = let (\* for any n\>=0 \*)
      val f = nreads(TS.mkInstream (rdr, ""),n)
      in 
        TS.closeIn f;
        size(#1(TS.input f)) = 0
      end

The sequence of strings returned from a fresh stream by [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) is exactly the sequence returned by the underlying reader. This includes end-of-stream conditions, which the reader indicates by returning a zero-element vector and [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) indicates in the same way.

The [`endOfStream`](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC) test is equivalent to [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) returning an empty sequence:

fun isEOS f = let 
      val (a,\_) = TS.input f  
      in 
        ((size a)=0) = (TS.endOfStream f)   
      end

The semantics of [`inputAll`](stream-io.md#SIG:STREAM_IO.inputAll:VAL:SPEC) can be defined in terms of [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC):

fun inputAll f  =
      case TS.input f of
        ("",f') =\> ("",f')
      \| (s,f') =\> let
          val (rest,f'') = inputAll f'
          in
            (s ^ rest, f'')
          end

An actual implementation, however, is likely to be much more efficient; for example, on a large file, [`inputAll`](stream-io.md#SIG:STREAM_IO.inputAll:VAL:SPEC) might read the whole file in a single system call or use memory mapping. Note that if a stream `f` contains data `"abc"` followed by an end-of-stream followed by `"defg"` and another end-of-stream, then `inputAll f` returns `("abc",f')`, and `inputAll f'` returns `("defg",f'')`.

The semantics of [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC) can be related to [`inputAll`](stream-io.md#SIG:STREAM_IO.inputAll:VAL:SPEC) by the following predicate:

fun allAndN (f,n) = let
      val (s,f1) = TS.inputN(f,n)
      val (t,f2)= TS.inputAll f
      in 
        size s \< n andalso s=t andalso equiv(f1,f2)
        orelse let 
          val (r,f3) = TS.inputAll f1
          in 
            size s = n andalso t = s ^ r andalso equiv(f2,f3)
          end
      end

where the `equiv` predicate represents that the two argument streams behave identically under [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC):

fun equiv (f,g) = let
      val (s,f') = TS.input f 
      val (t,g') = TS.input g
      in 
        s=t andalso equiv(f',g')
      end

ignoring termination conditions. If `f` contained exactly `n` characters before the end-of-stream, then `r` in `allAndN` will be the empty string. Another way of saying this is that [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC) returns fewer than `n` characters if and only if those elements are followed by an end-of-stream.

The semantics of [`input1`](stream-io.md#SIG:STREAM_IO.input1:VAL:SPEC) can be defined in terms of [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC):

fun input1 f =
      case TS.inputN (f,1) of 
        ("",\_) =\> NONE
      \| (s,f')=\> SOME(String.sub(s,0),f')

If `chunkSize` = 1 in the underlying [`reader`](prim-io.md#SIG:PRIM_IO.reader:TY:SPEC), then input operations should be unbuffered. Thus, the following function always returns true:

fun isTrue (rdr : TextPrimIO.reader) = let
      val f = TS.mkInstream(rdr, "")
      val (\_,f') = TS.input f
      val (TextPrimIO.RD{chunkSize,...},s) = TS.getReader f'
      in
        (chunkSize \> 1) orelse (size s = 0)
      end

where `rdr` denotes a [`reader`](prim-io.md#SIG:PRIM_IO.reader:TY:SPEC) created from a newly opened file. Although [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) may perform a Primitive I/O read operation on the reader for `k` \>= 1 elements, it must immediately return all the elements it receives. This does not hold, however, for partly determined input streams. For example, the function

fun maybeTrue (rdr : TextPrimIO.reader) = let
      val f = TS.mkInstream(rdr, "")
      val _ = TS.input (#2 (TS.input f))
      val (\_,f') = TS.input f
      val (TextPrimIO.RD{chunkSize,...},s) = TS.getReader f'
      in
        (chunkSize \> 1) orelse (size s = 0)
      end

might return false. In this case, the stream `f` has accumulated a history of more input, which will not be emptied by a single call to [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC).

Similarly, if a writer sets `chunkSize` = 1, it suggests that output operations should be unbuffered. An application can specify that a stream should be unbuffered using the [`setBufferMode`](stream-io.md#SIG:STREAM_IO.setBufferMode:VAL:SPEC) function.

> **Implementation note:**
>
> A general rule for implementing stream input is: \`\`_do not bother the reader.''_ Whenever it is possible to do so, input must be done by using elements from the buffer, without any operation on the underlying reader. This is necessary so that repeated calls to [`endOfStream`](stream-io.md#SIG:STREAM_IO.endOfStream:VAL:SPEC) will not make repeated system calls.
>
> Implementations may require a device such as an extra boolean to mark multiple end-of-streams, in order that [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC) applied to the same stream always returns the same vector.
>
> The manual page of the [`StreamIO`](stream-io-fn.md#StreamIO:FCT:SPEC) functor lists a variety of implementation suggestions, many of which are applicable to any implementation of the [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) signature.
>
> In general, if an exception occurs during any Stream I/O operation, then the stream must leave itself in a consistent state, without losing or duplicating data. In some SML systems, a user interrupt aborts execution and returns control to a top-level prompt, without raising any exception that the current execution can handle. It may be the case that some information must be lost or duplicated. Data (input or output) must _never_ be duplicated, but may be lost. This can be accomplished without Stream I/O doing any explicit masking of interrupts or locking. On output, the internal state (saying how much has been written) should be updated _before_ doing the _write_ operation; on input, the _read_ should be done before updating the count of valid characters in the buffer.
