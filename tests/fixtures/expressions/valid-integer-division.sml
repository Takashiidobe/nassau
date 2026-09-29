(* div rounds toward negative infinity. *)
val a = 7 div 2
val b = 7 div ~2
val c = ~7 div 2
val d = ~7 div ~2
val total = a * 1000 + (b + 10) * 100 + (c + 10) * 10 + d
val _ = Posix.Process.exit (Word8.fromInt (total div 100))
(* CHECK-EXIT: 36 *)
