# <span id="section:0"></span>The `IO` structure

---

#### Synopsis

<span id="IO:SIG:SPEC"></span>
<span id="IO:STR:SPEC"></span>

```sml
signature IO
structure IO :> IO
```

The `IO` structure contains types and values common to all the input/output structures and functors. In particular, it defines the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception, which is used to provide structured information for any errors occurring during I/O.

---

#### Interface

<span id="SIG:IO.Io:EXN:SPEC"></span>
<span id="SIG:IO.BlockingNotSupported:EXN:SPEC"></span>
<span id="SIG:IO.NonblockingNotSupported:EXN:SPEC"></span>
<span id="SIG:IO.RandomAccessNotSupported:EXN:SPEC"></span>
<span id="SIG:IO.ClosedStream:EXN:SPEC"></span>
<span id="SIG:IO.buffer_mode:TY:SPEC"></span>
<span id="SIG:IO.NO_BUF:TY:SPEC"></span>
<span id="SIG:IO.LINE_BUF:TY:SPEC"></span>
<span id="SIG:IO.BLOCK_BUF:TY:SPEC"></span>

```sml
exception Io of {
name : string,
function : string,
cause : exn
}
exception BlockingNotSupported
exception NonblockingNotSupported
exception RandomAccessNotSupported
exception ClosedStream
datatype buffer_mode = NO_BUF | LINE_BUF | BLOCK_BUF
```

#### Description

<span id="SIG:IO.Io:EXN"></span>**`exception`**` Io `**`of`**` {`
`  name `**`:`**` string,`
`  function `**`:`**` string,`
`  cause `**`:`**` exn`
`}`  
This is the principal exception raised when an error occurs in the I/O subsystem. The components of [`Io`](io.md#SIG:IO.Io:EXN:SPEC) are:

`name`  
The `name` component of the reader or writer.

`function`  
The name of the function raising the exception.

`cause`  
The underlying exception raised by the reader or writer, or detected at the stream I/O level.

Some of the standard causes are:

- [`OS.SysErr`](os.md#SIG:OS.SysErr:EXN:SPEC) if an actual system call was done and failed.
- [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) if ill-formed arguments are given.
- [`BlockingNotSupported`](io.md#SIG:IO.BlockingNotSupported:EXN:SPEC)
- [`NonblockingNotSupported`](io.md#SIG:IO.NonblockingNotSupported:EXN:SPEC)
- [`ClosedStream`](io.md#SIG:IO.ClosedStream:EXN:SPEC)

The `cause` field of [`Io`](io.md#SIG:IO.Io:EXN:SPEC) is not limited to these particular exceptions. Users who create their own readers or writers may raise any exception they like, which will be reported as the `cause` field of the resulting [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception.

<span id="SIG:IO.BlockingNotSupported:EXN"></span>**`exception`**` BlockingNotSupported`  
The exception used in the [`output`](stream-io.md#SIG:STREAM_IO.output:VAL:SPEC), [`outputSubstr`](text-stream-io.md#SIG:TEXT_STREAM_IO.outputSubstr:VAL:SPEC), [`output1`](stream-io.md#SIG:STREAM_IO.output1:VAL:SPEC), and [`flushOut`](stream-io.md#SIG:STREAM_IO.flushOut:VAL:SPEC) I/O operations if the underlying writer does not support blocking writes; or in the [`input`](stream-io.md#SIG:STREAM_IO.input:VAL:SPEC), [`inputN`](stream-io.md#SIG:STREAM_IO.inputN:VAL:SPEC), and [`input1`](stream-io.md#SIG:STREAM_IO.input1:VAL:SPEC) I/O operations if the underlying reader does not support blocking reads. It should never be raised within the I/O system; it should only be used in the `cause` field of an [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception.

<span id="SIG:IO.NonblockingNotSupported:EXN"></span>**`exception`**` NonblockingNotSupported`  
The exception used by the [`canInput`](stream-io.md#SIG:STREAM_IO.canInput:VAL:SPEC) I/O operation if the underlying stream does not support non-blocking input. It should never be raised within the I/O system; it should only be used in the `cause` field of an [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception.

<span id="SIG:IO.RandomAccessNotSupported:EXN"></span>**`exception`**` RandomAccessNotSupported`  
The exception used by the [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) position operations to indicate that random access operations are not supported by the underlying device. It should never be raised within the I/O system; it should only be used in the `cause` field of an [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception.

<span id="SIG:IO.ClosedStream:EXN"></span>**`exception`**` ClosedStream`  
This exception is used by the output I/O operations if the underlying object is closed or terminated. It should never be raised within the I/O system; it should only be used in the `cause` field of an [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception.

<span id="SIG:IO.buffer_mode:TY"></span>**`datatype`**` buffer_mode = NO_BUF | LINE_BUF | BLOCK_BUF`  
These values specify the type of buffering used on output streams. If an output stream has mode [`BLOCK_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC), the implementation should store output in a buffer, actually writing the buffer's content to the device only when the buffer is full. If an output stream has mode [`NO_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC), the implementation should write the argument bytes of any output function directly to the corresponding device. If an output stream has mode [`LINE_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC), output bytes should be buffered until a newline character (`#"\n"`) is seen, at which point the buffer should be flushed, including the newline character. For binary streams, [`LINE_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC) mode should be treated as a synonym for [`BLOCK_BUF`](io.md#SIG:IO.buffer_mode:TY:SPEC).

> **Implementation note:**
>
> Output buffering is provided for efficiency, to reduce the number of writes to the underlying device, which may be an expensive operation. The I/O subsystem should select the initial buffer mode based on the output device. By default, output should be buffered. The optimum buffer size is specified by the `chunkSize` field in the underlying [`writer`](prim-io.md#SIG:PRIM_IO.writer:TY:SPEC) value. Output to [`TextIO.stdErr`](text-io.md#SIG:TEXT_IO.stdErr:VAL:SPEC) should be unbuffered. Output to a terminal-like device should be line-buffered. A simple test for this is
>
> [OS.IO.kind](os-io.md#SIG:OS_IO.kind:VAL:SPEC) iod = [OS.IO.Kind.tty](os-io.md#SIG:OS_IO.Kind.tty:VAL:SPEC)
>
> where `iod` is the I/O descriptor associated with the open stream.


#### Examples

```repl
IO.Io {name="read", function="input", cause=Fail "example"};;
```

#### See Also

> [`BinIO`](bin-io.md#BinIO:STR:SPEC), [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC), [`PRIM_IO`](prim-io.md#PRIM_IO:SIG:SPEC), [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC), [`TextIO`](text-io.md#TextIO:STR:SPEC)

#### Discussion

The imperative, stream, and primitive I/O modules will never raise a bare [`BlockingNotSupported`](io.md#SIG:IO.BlockingNotSupported:EXN:SPEC), [`NonblockingNotSupported`](io.md#SIG:IO.NonblockingNotSupported:EXN:SPEC), or [`ClosedStream`](io.md#SIG:IO.ClosedStream:EXN:SPEC) exception; these exceptions are only used in the `cause` field of the [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception. Any module, however, may raise [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) directly if given ill-formed arguments, or may raise [`Io`](io.md#SIG:IO.Io:EXN:SPEC) with [`Subscript`](general.md#SIG:GENERAL.Subscript:EXN:SPEC) as the `cause`.

It is possible that multiple error conditions hold when an I/O function is called. For example, a random access call may be made on a closed stream corresponding to a device that does not support random access. The `cause` reported in the generated [`Io`](io.md#SIG:IO.Io:EXN:SPEC) exception is implementation-dependent.
