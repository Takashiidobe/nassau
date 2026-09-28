val arithmetic_first = if 1 + 2 < 4 then 10 else 20
val product_first = if 2 * 3 >= 6 then 1 else 2
val chained = if (1 < 2) = true then 3 else 4
val nested = if 1 <> 2 then if 3 > 4 then 5 else 6 else 7
val answer = arithmetic_first + product_first + chained + nested
val _ = Posix.Process.exit (Word8.fromInt answer)
(* CHECK-EXIT: 20 *)
