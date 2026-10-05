(* SML'97 grammar, pattern: layered. *)
fun pi n = print (Int.toString n ^ "\n")
fun f (l as x :: _) = x + length l | f [] = 0
fun g (op p : int * int as (a, _)) = a + #2 p
val _ = pi (f [5, 6] + g (1, 2))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 10 *)
