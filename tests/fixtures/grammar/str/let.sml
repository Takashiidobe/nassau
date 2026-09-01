(* SML'97 grammar, structure expression: let. *)
fun pi n = print (Int.toString n ^ "\n")
structure A = let val h = 3 structure B = struct val x = h end in struct val y = B.x + 1 end end
val _ = pi A.y
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
