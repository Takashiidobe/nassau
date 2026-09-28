val product_first = 2 + 3 * 4
val subtract_left = 20 - 5 - 3
val divide_left = 100 div 5 div 2
val mixed = 2 + 3 * 4 - 10 div 3
val answer = product_first + subtract_left + divide_left + mixed
val _ = Posix.Process.exit (Word8.fromInt answer)
(* CHECK-EXIT: 47 *)
