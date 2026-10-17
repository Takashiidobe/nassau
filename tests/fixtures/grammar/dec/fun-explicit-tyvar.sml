(* SML'97 grammar, declaration: fun explicit tyvar (nassau-ugc.14). *)
fun pi n = print (Int.toString n ^ "\n")
fun 'a id (x : 'a) = x
val _ = pi (id 1)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
