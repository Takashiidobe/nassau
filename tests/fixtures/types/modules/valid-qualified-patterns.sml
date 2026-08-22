structure S = struct
  datatype d = A | B of int
  datatype 'a tree = Leaf | Node of 'a tree * 'a * 'a tree
  exception Oops of int
end
fun f S.A = 0 | f (S.B n) = n
fun size S.Leaf = 0 | size (S.Node (l, _, r)) = size l + 1 + size r
val a = f (S.B 3)
val b = size (S.Node (S.Leaf, "x", S.Node (S.Leaf, "y", S.Leaf)))
val c = case S.B 1 of S.B n => n | S.A => 0
val d = (raise S.Oops 1) handle S.Oops n => n
val e = fn (S.B n) => n | S.A => 0
val S.B g = S.B 1
(* CHECK-STDOUT: val f : S.d -> int *)
(* CHECK-STDOUT-NEXT: val size : 'a S.tree -> int *)
(* CHECK-STDOUT-NEXT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : int *)
(* CHECK-STDOUT-NEXT: val d : int *)
(* CHECK-STDOUT-NEXT: val e : S.d -> int *)
(* CHECK-STDOUT-NEXT: val g : int *)
(* CHECK-RUN-EXIT: 0 *)
