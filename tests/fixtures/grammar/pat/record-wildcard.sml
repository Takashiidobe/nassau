(* SML'97 grammar, pattern: record wildcard. *)
fun pi n = print (Int.toString n ^ "\n")
fun f ({a, ...} : {a : int, b : int}) = a
fun g ({b = x, c = _, ...} : {a : int, b : int, c : int}) = x
val _ = pi (f {a = 1, b = 2} + g {a = 0, b = 5, c = 9})
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 *)
