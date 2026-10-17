(* SML'97 grammar, declaration: fun explicit tyvars (nassau-ugc.14). *)
fun pi n = print (Int.toString n ^ "\n")
fun ('a, 'b) fst (x : 'a, _ : 'b) = x
val _ = pi (fst (1, true))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
