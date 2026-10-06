# <span id="section:0"></span>The `TEXT_STREAM_IO` signature

---

#### Synopsis

<span id="TEXT_STREAM_IO:SIG:SPEC"></span>

```sml
signature TEXT_STREAM_IO
```

The signature `TEXT_STREAM_IO` extends the `STREAM_IO` signature to accommodate text I/O. In particular, it binds the I/O element to [`Char.char`](char.md#SIG:CHAR.char:TY:SPEC), and provides several text-based I/O operations.

---

#### Interface

<span id="SIG:TEXT_STREAM_IO.inputLine:VAL:SPEC"></span>
<span id="SIG:TEXT_STREAM_IO.outputSubstr:VAL:SPEC"></span>

```sml
include STREAM_IO
where type vector = CharVector.vector
where type elem = Char.char
val inputLine : instream -> (string * instream) option
val outputSubstr : outstream * substring -> unit
```

#### Description

<span id="SIG:TEXT_STREAM_IO.inputLine:VAL"></span>

### `inputLine`

```sml
val inputLine : instream -> (string * instream) option
```
`inputLine ``strm`` `  
returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``ln``, ``strm'``)`, where `ln` is the next line of input in the stream `strm` and `strm'` is the residual stream. Specifically, `ln` returns all characters from the current position up to and including the next newline (`#"\n"`) character. If it detects an end-of-stream before the next newline, it returns the characters read appended with a newline. Thus, `ln` is guaranteed to always be new-line terminated (and thus nonempty). If the current stream position is the end-of-stream, then it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). It raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the length of the line exceeds the length of the longest string.



```repl
val input = TextIO.getInstream (TextIO.openString "hello\n");;
TextIO.StreamIO.inputLine input;; (* SOME ("hello\n", rest) *)
```
<span id="SIG:TEXT_STREAM_IO.outputSubstr:VAL"></span>

### `outputSubstr`

```sml
val outputSubstr : outstream * substring -> unit
```
`outputSubstr (``strm``, ``ss``) `  
outputs the substring `ss` to the text stream `strm`. This is equivalent to:

output (`strm`, [Substring.string](substring.md#SIG:SUBSTRING.string:VAL:SPEC) `ss`)




```repl
TextIO.StreamIO.outputSubstr (TextIO.getOutstream TextIO.stdOut, Substring.full "hello");; (* writes hello *)
```
#### Examples

```repl
TextIO.StreamIO.inputLine (TextIO.getInstream (TextIO.openString ""));; (* NONE *)
```

#### See Also

> [`BinIO`](bin-io.md#BinIO:STR:SPEC), [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC), [`OS.Path`](os.md#SIG:OS.Path:STR:SPEC)
