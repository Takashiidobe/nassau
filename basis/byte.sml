structure Byte =
struct
  fun byteToChar b = Char.chr (Word8.toInt b)
  fun charToByte c = Word8.fromInt (Char.ord c)
end
