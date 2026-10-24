structure Byte =
struct
  fun byteToChar b = Char.chr (Word8.toInt b)
  fun charToByte c = Word8.fromInt (Char.ord c)
  fun bytesToString v = CharVector.tabulate (Word8Vector.length v, fn i => byteToChar (Word8Vector.sub (v, i)))
  fun stringToBytes s = Word8Vector.tabulate (String.size s, fn i => charToByte (String.sub (s, i)))
  fun unpackStringVec slice =
    CharVector.tabulate (Word8VectorSlice.length slice, fn i => byteToChar (Word8VectorSlice.sub (slice, i)))
  fun unpackString slice =
    CharVector.tabulate (Word8ArraySlice.length slice, fn i => byteToChar (Word8ArraySlice.sub (slice, i)))
  fun packString (array, start, substring) =
    let
      val (s, offset, count) = Substring.base substring
      fun loop i =
        if i = count then ()
        else (Word8Array.update (array, start + i, charToByte (String.sub (s, offset + i))); loop (i + 1))
    in
      if start < 0 orelse start + count > Word8Array.length array then raise Subscript
      else loop 0
    end
end
