functor ImperativeIO (
  structure StreamIO : STREAM_IO
  structure Vector : MONO_VECTOR
  structure Array : MONO_ARRAY
  sharing type StreamIO.elem = Vector.elem = Array.elem
  sharing type StreamIO.vector = Vector.vector = Array.vector
) : IMPERATIVE_IO =
struct
  structure StreamIO = StreamIO
  type vector = StreamIO.vector
  type elem = StreamIO.elem
  type instream = StreamIO.instream ref
  type outstream = StreamIO.outstream ref

  fun mkInstream stream = ref stream
  fun getInstream stream = !stream
  fun setInstream (stream, value) = stream := value
  fun mkOutstream stream = ref stream
  fun getOutstream stream = !stream
  fun setOutstream (stream, value) = stream := value

  fun input stream =
    let val (value, rest) = StreamIO.input (!stream)
    in stream := rest; value end

  fun inputN (stream, count) =
    if count < 0 orelse count > Vector.maxLen then raise Size
    else let val (value, rest) = StreamIO.inputN (!stream, count)
         in stream := rest; value end

  fun input1 stream =
    let val value = inputN (stream, 1)
    in if Vector.length value = 0 then NONE else SOME (Vector.sub (value, 0)) end

  fun inputAll stream =
    let val (value, rest) = StreamIO.inputAll (!stream)
    in stream := rest; value end

  fun canInput (stream, count) =
    if count < 0 then raise Size else StreamIO.canInput (!stream, count)

  fun lookahead stream =
    case StreamIO.input1 (!stream) of
      NONE => NONE
    | SOME (value, _) => SOME value

  fun closeIn stream = StreamIO.closeIn (!stream)
  fun endOfStream stream = StreamIO.endOfStream (!stream)
  fun output (stream, value) = StreamIO.output (!stream, value)
  fun output1 (stream, value) = StreamIO.output1 (!stream, value)
  fun flushOut stream = StreamIO.flushOut (!stream)
  fun closeOut stream = StreamIO.closeOut (!stream)
  fun getPosOut stream = StreamIO.getPosOut (!stream)
  fun setPosOut (stream, position) = stream := StreamIO.setPosOut position
end
