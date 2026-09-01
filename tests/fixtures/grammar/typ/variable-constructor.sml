(* SML'97 grammar, type: variable constructor. *)
fun pi n = print (Int.toString n ^ "\n")
type ('a, 'b) pair = 'a * 'b
val p : (int, string) pair = (1, "a")
val q : int list option = SOME [1]
val _ = pi (#1 p)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
