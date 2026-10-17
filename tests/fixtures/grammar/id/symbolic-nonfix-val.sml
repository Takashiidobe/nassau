(* SML'97 grammar, identifier: symbolic nonfix val (nassau-ugc.13). *)
fun pi n = print (Int.toString n ^ "\n")
val <+> = fn (a, b) => a + b
val _ = pi (<+> (1, 2))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
