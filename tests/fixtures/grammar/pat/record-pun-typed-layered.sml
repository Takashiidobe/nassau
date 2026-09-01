(* SML'97 grammar, pattern: record pun typed layered. *)
fun pi n = print (Int.toString n ^ "\n")
fun f {a : int, b as (x, _)} = a + x + #2 b
val _ = pi (f {a = 1, b = (2, 3)})
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 *)
