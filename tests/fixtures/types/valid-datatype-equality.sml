datatype color = Red | Green
datatype 'a box = Box of 'a
datatype 'a tree = Leaf | Node of 'a tree * 'a * 'a tree
datatype opaque = Wrap of int -> int
datatype 'a maybe = Nothing | Just of 'a
val a = Red = Green
val b = Box 1 = Box 2
val c = Node (Leaf, "x", Leaf) = Leaf
fun same (x, y) = Box x = Box y
fun has (x, t) = t = Node (Leaf, x, Leaf)
val d = Just (Box Red) <> Nothing
val r = ref Red = ref Green
(* CHECK-STDOUT: val a : bool *)
(* CHECK-STDOUT-NEXT: val b : bool *)
(* CHECK-STDOUT-NEXT: val c : bool *)
(* CHECK-STDOUT-NEXT: val same : ''a * ''a -> bool *)
(* CHECK-STDOUT-NEXT: val has : ''a * ''a tree -> bool *)
(* CHECK-STDOUT-NEXT: val d : bool *)
(* CHECK-STDOUT-NEXT: val r : bool *)
