(* SML'97 grammar, declaration: abstype withtype. *)
fun pi n = print (Int.toString n ^ "\n")
abstype stack = S of elems withtype elems = int list
with val empty = S [] fun push (x, S l) = S (x :: l) fun size (S l) = length l end
val _ = pi (size (push (1, empty)))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
