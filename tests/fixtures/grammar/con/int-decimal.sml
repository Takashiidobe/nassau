(* SML'97 grammar, constant: int decimal. *)
fun pi n = print (Int.toString n ^ "\n")
val _ = pi 42 val _ = pi ~7
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 42 *)
(* CHECK-STDOUT-NEXT: ~7 *)
