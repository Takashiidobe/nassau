(* SML'97 grammar, structure expression: strbind annotated and. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig val x : int end
structure A : S = struct val x = 1 val hidden = 2 end and B :> S = struct val x = 2 end
val _ = pi (A.x + B.x)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
