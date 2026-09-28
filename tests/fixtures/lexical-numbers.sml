val hexadecimal = 0x10
val negative = ~2
val decimalReal = 1.2e~3 + 1e~3
val answer = hexadecimal + negative
val _ = Posix.Process.exit (Word8.fromInt answer)
