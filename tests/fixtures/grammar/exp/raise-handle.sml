(* SML'97 grammar, expression: raise handle. *)
fun pi n = print (Int.toString n ^ "\n")
exception E of int
val _ = pi ((raise E 4) handle E n => n + 1 | _ => 0)
val _ = pi (1 handle E _ => 2)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 5 *)
(* CHECK-STDOUT-NEXT: 1 *)
