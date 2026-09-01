(* SML'97 grammar, structure expression: functor app str. *)
fun pi n = print (Int.toString n ^ "\n")
functor F (X : sig val x : int end) = struct val y = X.x + 1 end
structure A = F (struct val x = 1 end)
structure B = struct val x = 10 end structure C = F (B)
val _ = pi (A.y + C.y)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 13 *)
