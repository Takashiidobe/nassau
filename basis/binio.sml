structure BinIO =
struct
  type vector = Word8Vector.vector
  type elem = Word8.word
  type instream = TextIO.instream
  type outstream = TextIO.outstream

  fun rename (function, action) =
    action () handle IO.Io {name, cause, ...} => raise IO.Io {name = name, function = function, cause = cause}

  fun bytes s = Byte.stringToBytes s
  fun byte c = Word8.fromInt (Prim.ord c)

  fun openIn path = rename ("BinIO.openIn", fn () => TextIO.openIn path)
  fun openOut path = rename ("BinIO.openOut", fn () => TextIO.openOut path)
  fun openAppend path = rename ("BinIO.openAppend", fn () => TextIO.openAppend path)

  val closeIn = TextIO.closeIn
  val closeOut = TextIO.closeOut
  fun input s = rename ("BinIO.input", fn () => bytes (TextIO.input s))
  fun input1 s = rename ("BinIO.input1", fn () => Option.map byte (TextIO.input1 s))
  fun inputN (s, n) = rename ("BinIO.inputN", fn () => bytes (TextIO.inputN (s, n)))
  fun inputAll s = rename ("BinIO.inputAll", fn () => bytes (TextIO.inputAll s))
  fun canInput (s, n) = TextIO.canInput (s, n)
  fun lookahead s = rename ("BinIO.lookahead", fn () => Option.map byte (TextIO.lookahead s))
  val endOfStream = TextIO.endOfStream
  fun output (s, v) = rename ("BinIO.output", fn () => TextIO.output (s, Byte.bytesToString v))
  fun output1 (s, w) = output (s, Word8Vector.fromList [w])
  val flushOut = TextIO.flushOut
end
