(* SML'97 grammar, pattern: constants. *)
fun pi n = print (Int.toString n ^ "\n")
fun f 0 = 1 | f _ = 2
fun g "a" = 3 | g _ = 4
fun h #"c" = 5 | h _ = 6
fun w 0w1 = 7 | w _ = 8
val _ = pi (f 0 + g "a" + h #"c" + w 0w1)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 16 *)
