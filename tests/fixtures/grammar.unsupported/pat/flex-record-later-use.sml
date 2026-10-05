(* SML'97 grammar, pattern: flex record later use (nassau-ugc.21). *)
fun pi n = print (Int.toString n ^ "\n")
fun f {a, ...} = a + 0
val _ = pi (f {a = 1, b = 2})
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
