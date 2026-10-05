(* SML'97 grammar, constant: int hex. *)
fun pi n = print (Int.toString n ^ "\n")
val _ = pi 0x2A val _ = pi ~0x1f val _ = pi 0xFF
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 42 *)
(* CHECK-STDOUT-NEXT: ~31 *)
(* CHECK-STDOUT-NEXT: 255 *)
