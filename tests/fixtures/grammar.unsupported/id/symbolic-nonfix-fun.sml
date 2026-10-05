(* SML'97 grammar, identifier: symbolic nonfix fun (nassau-ugc.13). *)
fun pi n = print (Int.toString n ^ "\n")
fun !%& x = x + 1
val _ = pi (!%& 1)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
