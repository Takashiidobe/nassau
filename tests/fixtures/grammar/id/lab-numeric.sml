(* SML'97 grammar, identifier: lab numeric. *)
fun pi n = print (Int.toString n ^ "\n")
val r = {1 = 10, 2 = 20}
val (a, b) = r
val _ = pi (#2 r + a + b)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 50 *)
