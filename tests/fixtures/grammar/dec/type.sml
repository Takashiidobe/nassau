(* SML'97 grammar, declaration: type. *)
fun pi n = print (Int.toString n ^ "\n")
type t = int and 'a u = 'a * 'a type ('a, 'b) v = 'a -> 'b
val x : t u = (1, 2) val f : (int, int) v = fn y => y val _ = pi (f (#1 x))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
