(* SML'97 grammar, structure expression: functor app empty dec. *)
fun pi n = print (Int.toString n ^ "\n")
functor F () = struct val y = 5 end
structure A = F () val _ = pi A.y
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 5 *)
