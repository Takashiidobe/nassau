(* SML'97 grammar, identifier: longid. *)
fun pi n = print (Int.toString n ^ "\n")
structure S = struct structure T = struct val x = 7 end end
val _ = pi S.T.x
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 7 *)
