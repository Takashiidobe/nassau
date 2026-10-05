(* SML'97 grammar, identifier: symbolic infix. *)
fun pi n = print (Int.toString n ^ "\n")
infix <+>
fun a <+> b = a + b * 10
val _ = pi (1 <+> 2)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 21 *)
