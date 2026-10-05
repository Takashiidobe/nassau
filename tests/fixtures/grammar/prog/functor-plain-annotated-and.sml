(* SML'97 grammar, program: functor plain annotated and. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig val x : int end
functor F (X : S) : S = struct val x = X.x + 1 end and G (Y : S) :> S = struct val x = Y.x * 2 end
structure A = G (F (struct val x = 1 end)) val _ = pi A.x
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
