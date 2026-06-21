(* exit_code: 18 *)
val _ = print "hello, world\n"
val quotient = 24 div 3
val product = quotient * 4
val answer = 42 + 8 - product
val half = 1.0 / 2.0
val one = half + half
val _ = Posix.Process.exit (Word8.fromInt answer)
