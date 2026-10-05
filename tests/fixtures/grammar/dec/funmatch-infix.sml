(* SML'97 grammar, declaration: funmatch infix. *)
fun pi n = print (Int.toString n ^ "\n")
infix ++
fun 0 ++ y = y
  | x ++ y = x * 10 + y
val _ = pi (1 ++ 2)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 12 *)
