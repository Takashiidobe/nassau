(* SML'97 grammar, declaration: funmatch result type. *)
fun pi n = print (Int.toString n ^ "\n")
fun f (x : int) : int = x + 1
fun g 0 : int list = [] | g n = [n] val _ = pi (f 1 + length (g 2))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
