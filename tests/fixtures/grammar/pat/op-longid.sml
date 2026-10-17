(* SML'97 grammar, pattern: op longid (nassau-ugc.15). *)
fun pi n = print (Int.toString n ^ "\n")
structure S = struct datatype t = C of int end
val op S.C m = S.C 3
val _ = pi m
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
