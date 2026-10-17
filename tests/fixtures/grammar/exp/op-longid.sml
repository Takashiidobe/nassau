(* SML'97 grammar, expression: op longid (nassau-ugc.15). *)
fun pi n = print (Int.toString n ^ "\n")
structure S = struct fun f (a, b) = a - b end
val _ = pi (op S.f (5, 1))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
