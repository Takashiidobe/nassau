val _ = print "\a\b\t\n\v\f\r\\\"\^A\065"
val _ = Posix.Process.exit (Word8.fromInt 0)
