(* SML'97 grammar, declaration: datatype withtype. *)
fun pi n = print (Int.toString n ^ "\n")
datatype tree = Leaf | Node of forest withtype forest = tree list
fun size Leaf = 1 | size (Node f) = foldl (fn (t, a) => a + size t) 1 f
val _ = pi (size (Node [Leaf, Leaf]))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
