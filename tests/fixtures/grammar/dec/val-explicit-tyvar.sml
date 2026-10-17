(* SML'97 grammar, declaration: val explicit tyvar (nassau-ugc.14). *)
fun pi n = print (Int.toString n ^ "\n")
val 'a id = fn (x : 'a) => x
val _ = pi (id 1)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
