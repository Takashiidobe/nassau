(* SML'97 grammar, expression: fn. *)
fun pi n = print (Int.toString n ^ "\n")
val f = fn 0 => 10 | n => n
val _ = pi (f 0 + f 5)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 15 *)
