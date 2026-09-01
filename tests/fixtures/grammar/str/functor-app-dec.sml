(* SML'97 grammar, structure expression: functor app dec. *)
fun pi n = print (Int.toString n ^ "\n")
functor F (X : sig val x : int end) = struct val y = X.x + 1 end
structure A = F (val x = 4)
val _ = pi A.y
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 5 *)
