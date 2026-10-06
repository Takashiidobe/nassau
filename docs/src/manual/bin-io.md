# <span id="section:0"></span>The `BinIO` structure

---

#### Synopsis

<span id="BIN_IO:SIG:SPEC"></span>
<span id="BinIO:STR:SPEC"></span>

```sml
signature BIN_IO
structure BinIO :> BIN_IO
```

The structure `BinIO` provides input/output of binary data (8-bit bytes). The semantics of the various I/O operations can be found in the description of the [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC) signature. The [`openIn`](bin-io.md#SIG:BIN_IO.openIn:VAL:SPEC) and [`openOut`](bin-io.md#SIG:BIN_IO.openOut:VAL:SPEC) functions allow the creation of binary streams to read and write file data. Certain implementations may provide other ways to open files in structures specific to an operating system.

---

#### Interface

<span id="SIG:BIN_IO.openIn:VAL:SPEC"></span>
<span id="SIG:BIN_IO.openOut:VAL:SPEC"></span>
<span id="SIG:BIN_IO.openAppend:VAL:SPEC"></span>

```sml
include IMPERATIVE_IO
where type StreamIO.vector = Word8Vector.vector
where type StreamIO.elem = Word8.word
where type StreamIO.reader = BinPrimIO.reader
where type StreamIO.writer = BinPrimIO.writer
where type StreamIO.pos = BinPrimIO.pos
val openIn : string -> instream
val openOut : string -> outstream
val openAppend : string -> outstream
```

#### Description

<span id="SIG:BIN_IO.openIn:VAL"></span>

### `openIn`

```sml
val openIn : string -> instream
```

### `openOut`

```sml
val openOut : string -> outstream
```
These functions open the file named `name` for input and output, respectively. If `name` is a relative pathname, the file opened depends on the current working directory. With [`openOut`](bin-io.md#SIG:BIN_IO.openOut:VAL:SPEC), the file is created if it does not already exist and truncated to length zero otherwise. These raise [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if a stream cannot be opened on the given file or, in the case of [`openIn`](bin-io.md#SIG:BIN_IO.openIn:VAL:SPEC), the file `name` does not exist.


```repl
val setup = BinIO.openOut "basis-example.bin";
BinIO.closeOut setup;
val input = BinIO.openIn "basis-example.bin";; (* instream *)
BinIO.closeIn input;; (* () *)
```
<span id="SIG:BIN_IO.openAppend:VAL"></span>

### `openAppend`

```sml
val openAppend : string -> outstream
```
opens the file named `name` for output in append mode, creating it if it does not already exist. If the file already exists, it sets the current position at the end of the file. It raises [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if a stream cannot be opened on the given file.

Beyond having the initial file position at the end of the file, any additional properties are system and implementation dependent. On operating systems (_e.g._, Unix) that support \`\`atomic append mode,'' each (flushed) output operation to the file will be appended to the end, even if there are other processes writing to the file simultaneously. Due to buffering, however, writing on an [`outstream`](imperative-io.md#SIG:IMPERATIVE_IO.outstream:TY:SPEC) need not be atomic, _i.e._, output from a different process may interleave the output of a single write using the stream library. On certain other operating systems, having the file open for writing prevents any other process from opening the file for writing.

```repl
val setup = BinIO.openOut "basis-example.bin";
BinIO.closeOut setup;
val output = BinIO.openAppend "basis-example.bin";; (* outstream positioned at end *)
BinIO.closeOut output;; (* () *)
```
#### See Also

> [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC), [`OS.Path`](os.md#SIG:OS.Path:STR:SPEC), [`Posix.FileSys`](posix.md#SIG:POSIX.FileSys:STR:SPEC), [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC), [`TextIO`](text-io.md#TextIO:STR:SPEC)

#### Discussion

All streams created by [`mkInstream`](imperative-io.md#SIG:IMPERATIVE_IO.mkInstream:VAL:SPEC), [`mkOutstream`](imperative-io.md#SIG:IMPERATIVE_IO.mkOutstream:VAL:SPEC), and the open functions in [`BinIO`](bin-io.md#BinIO:STR:SPEC) will be closed (and the output streams among them flushed) when the SML program exits.

Note that the [`BinIO.StreamIO.pos`](stream-io.md#SIG:STREAM_IO.pos:TY:SPEC) type, equal to the [`BinPrimIO.pos`](prim-io.md#SIG:PRIM_IO.pos:TY:SPEC) type, is concrete, being a synonym for [`Position.int`](integer.md#SIG:INTEGER.int:TY:SPEC).
