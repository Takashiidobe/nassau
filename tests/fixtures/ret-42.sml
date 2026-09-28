val _ = print "hello, world\n"
val quotient = 24 div 3
val product = quotient * 4
val answer = if quotient >= 8 then 42 + 8 - product else 0
val half = 1.0 / 2.0
val one = half + half
val greater = 3 > 2
val less = 1 < 2
val equal = quotient = 8
val realGreater = 2.0 > 1.0
val realLess = 1.0 < 2.0
val greaterOrEqual = quotient >= 8
val lessOrEqual = quotient <= 8
val notEqual = quotient <> 9
val _ = Posix.Process.exit (Word8.fromInt answer)
(* CHECK-EXIT: 18 *)
(* CHECK-STDOUT: hello, world *)
