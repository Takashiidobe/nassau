# <span id="section:0"></span>The `TEXT_IO` signature

---

#### Synopsis

<span id="TEXT_IO:SIG:SPEC"></span>
<span id="TextIO:STR:SPEC"></span>
<span id="WideTextIO:STR:SPEC"></span>

```sml
signature TEXT_IO
structure TextIO :> TEXT_IO
structure WideTextIO :> TEXT_IO (* OPTIONAL *)
```

The `TEXT_IO` interface provides input/output of characters and strings. Most of the operations themselves are defined in the [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC) signature.

The [`TEXT_IO`](text-io.md#TEXT_IO:SIG:SPEC) signature is matched by two structures, the required [`TextIO`](text-io.md#TextIO:STR:SPEC) and the optional [`WideTextIO`](text-io.md#WideTextIO:STR:SPEC). The former implements strings based on the extended ASCII 8-bit characters. The latter provides strings of characters of some size greater than or equal to 8 bits.

The signature given below for [`TEXT_IO`](text-io.md#TEXT_IO:SIG:SPEC) is not valid SML, in that the substructure `StreamIO` is respecified. (It is initially specified as a substructure having signature [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC) in the included signature [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC).) This abuse of notation seems acceptable in that the intended meaning is clear (a structure matching [`TEXT_IO`](text-io.md#TEXT_IO:SIG:SPEC) also matches [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC) and has a substructure `StreamIO` that matches [`TEXT_STREAM_IO`](text-stream-io.md#TEXT_STREAM_IO:SIG:SPEC)) while avoiding a textual inclusion of the whole signature of [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC) except its `StreamIO` substructure.

---

#### Interface

<span id="SIG:TEXT_IO.inputLine:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.outputSubstr:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.openIn:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.openOut:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.openAppend:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.openString:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.stdIn:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.stdOut:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.stdErr:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.print:VAL:SPEC"></span>
<span id="SIG:TEXT_IO.scanStream:VAL:SPEC"></span>

```sml
include IMPERATIVE_IO
structure StreamIO : TEXT_STREAM_IO
where type reader = TextPrimIO.reader
where type writer = TextPrimIO.writer
where type pos = TextPrimIO.pos
val inputLine : instream -> string option
val outputSubstr : outstream * substring -> unit
val openIn : string -> instream
val openOut : string -> outstream
val openAppend : string -> outstream
val openString : string -> instream
val stdIn : instream
val stdOut : outstream
val stdErr : outstream
val print : string -> unit
val scanStream : ((Char.char, StreamIO.instream)
StringCvt.reader -> ('a, StreamIO.instream)
StringCvt.reader) -> instream -> 'a option
```

#### Description

<span id="SIG:TEXT_IO.inputLine:VAL"></span>
`inputLine ``strm`` `  
returns [`SOME`](option.md#SIG:OPTION.option:TY:SPEC)`(``ln``)`, where `ln` is the next line of input in the stream `strm`. Specifically, `ln` returns all characters from the current position up to and including the next newline (`#"\n"`) character. If it detects an end-of-stream before the next newline, it returns the characters read appended with a newline. Thus, `ln` is guaranteed to always be new-line terminated (and thus nonempty). If the current stream position is the end-of-stream, then it returns [`NONE`](option.md#SIG:OPTION.option:TY:SPEC). It raises [`Size`](general.md#SIG:GENERAL.Size:EXN:SPEC) if the length of the line exceeds the length of the longest string.

<span id="SIG:TEXT_IO.outputSubstr:VAL"></span>
`outputSubstr (``strm``, ``ss``) `  
outputs the substring `ss` to the text stream `strm`. This is equivalent to:

output (`strm`, [Substring.string](substring.md#SIG:SUBSTRING.string:VAL:SPEC) `ss`)


<span id="SIG:TEXT_IO.openIn:VAL"></span>
`openIn ``name`` `
` openOut ``name`` `  
These open the file named `name` for input and output, respectively. If `name` is a relative pathname, the file opened depends on the current working directory. On [`openOut`](text-io.md#SIG:TEXT_IO.openOut:VAL:SPEC), the file is created if it does not already exist and truncated to length zero otherwise. It raises [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if a stream cannot be opened on the given file, or in the case of [`openIn`](text-io.md#SIG:TEXT_IO.openIn:VAL:SPEC), the file `name` does not exist.

<span id="SIG:TEXT_IO.openAppend:VAL"></span>
`openAppend ``name`  
opens the file named `name` for output in append mode, creating it if it does not already exist. If the file already exists, the file pointer is positioned at the end of the file. It raises [`Io`](io.md#SIG:IO.Io:EXN:SPEC) if a stream cannot be opened on the given file.

Beyond having the initial file position be at the end of the file, any additional properties are system and implementation dependent. On operating systems (_e.g._, Unix) that support \`\`atomic append mode,'' each (flushed) output operation to the file will be appended to the end, even if there are other processes writing to the file simultaneously. Due to buffering, however, these writes need not be atomic, _i.e._, output from a different process may interleave the output of a single write using the stream library. On certain other operating systems, having the file open for writing prevents any other process from opening the file for writing.

<span id="SIG:TEXT_IO.openString:VAL"></span>
`openString ``s`` `  
creates an input stream whose content is `s`.

<span id="SIG:TEXT_IO.stdIn:VAL"></span>**`val`**` stdIn `**`:`**` instream`
**`val`**` stdOut `**`:`**` outstream`
**`val`**` stdErr `**`:`**` outstream`  
These correspond to the standard input, output, and error streams, respectively.

<span id="SIG:TEXT_IO.print:VAL"></span>
`print ``s`` `  
prints the string `s` to the standard output stream and flushes the stream. No newline character is appended.

This is available in the top-level environment as `print`. This is equivalent to:

(output ([stdOut](text-io.md#SIG:TEXT_IO.stdOut:VAL:SPEC), `s`); flushOut [stdOut](text-io.md#SIG:TEXT_IO.stdOut:VAL:SPEC))


<span id="SIG:TEXT_IO.scanStream:VAL"></span>
`scanStream ``scanFn`` ``strm`` `  
converts a stream-based scan function into one that works on Imperative I/O streams. For example, to attempt to scan a decimal integer from `stdIn`, one could use

scanStream (Int.scan StringCvt.DEC) stdIn

The function can be implemented as:

fun scanStream scanFn strm = let
      val instrm = getInstream strm
      in
    case (scanFn StreamIO.input1 instrm)
     of NONE =\> NONE
      \| SOME(v, instrm') =\> (
          setInstream (strm, instrm');
          SOME v)
      end

In addition to providing a convenient way to use Stream I/O scanning functions with Imperative I/O, the [`scanStream`](text-io.md#SIG:TEXT_IO.scanStream:VAL:SPEC) assures that input is not inadvertently lost due to lookahead during scanning.

#### Examples

```repl
TextIO.print "Hello from Nassau\n";;
```

#### See Also

> [`IMPERATIVE_IO`](imperative-io.md#IMPERATIVE_IO:SIG:SPEC), [`OS.Path`](os.md#SIG:OS.Path:STR:SPEC), [`STREAM_IO`](stream-io.md#STREAM_IO:SIG:SPEC), [`TEXT`](text.md#TEXT:SIG:SPEC), [`TEXT_STREAM_IO`](text-stream-io.md#TEXT_STREAM_IO:SIG:SPEC), [`TextPrimIO`](prim-io.md#TextPrimIO:STR:SPEC)

#### Discussion

All streams created by [`mkInstream`](imperative-io.md#SIG:IMPERATIVE_IO.mkInstream:VAL:SPEC), [`mkOutstream`](imperative-io.md#SIG:IMPERATIVE_IO.mkOutstream:VAL:SPEC), and the open functions in [`TextIO`](text-io.md#TextIO:STR:SPEC) will be closed (and the output streams among them flushed) when the SML program exits. The output streams [`TextIO.stdOut`](text-io.md#SIG:TEXT_IO.stdOut:VAL:SPEC) and [`TextIO.stdErr`](text-io.md#SIG:TEXT_IO.stdErr:VAL:SPEC) will be flushed, but not closed, on program exit.

When opening a stream for writing, the stream will be block buffered by default, unless the underlying file is associated with an interactive or terminal device (_i.e._, the kind of the underlying `iodesc` is `OS.IO.Kind.tty`), in which case the stream will be line buffered. Similarly, [`stdOut`](text-io.md#SIG:TEXT_IO.stdOut:VAL:SPEC) will be line buffered in the interactive case, but may be block buffered otherwise. [`stdErr`](text-io.md#SIG:TEXT_IO.stdErr:VAL:SPEC) is initially unbuffered.

The [`openIn`](text-io.md#SIG:TEXT_IO.openIn:VAL:SPEC), [`openOut`](text-io.md#SIG:TEXT_IO.openOut:VAL:SPEC), and [`openAppend`](text-io.md#SIG:TEXT_IO.openAppend:VAL:SPEC) functions allow creation of text streams. Certain implementations may provide other ways to open files in structures specific to an operating system. In such cases, there should be related functions for converting the open file into a value compatible with the Basis I/O subsystem. For example, the [`Posix.IO`](posix.md#SIG:POSIX.IO:STR:SPEC) defines the function [`mkTextWriter`](posix-io.md#SIG:POSIX_IO.mkTextWriter:VAL:SPEC), which generates a [`TextPrimIO.writer`](prim-io.md#SIG:PRIM_IO.writer:TY:SPEC) value from a POSIX file descriptor. The [`TextIO.StreamIO.mkOutstream`](stream-io.md#SIG:STREAM_IO.mkOutstream:VAL:SPEC) function can use that value to produces an output stream.
