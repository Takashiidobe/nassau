functor Tree (X : sig type key val less : key * key -> bool end) = struct
  datatype tree = Leaf | Node of tree * X.key * tree
  fun insert (k, Leaf) = Node (Leaf, k, Leaf)
    | insert (k, t as Node (l, k2, r)) =
        if X.less (k, k2) then Node (insert (k, l), k2, r)
        else if X.less (k2, k) then Node (l, k2, insert (k, r))
        else t
  fun size Leaf = 0
    | size (Node (l, _, r)) = size l + 1 + size r
end
structure IntTree = Tree (struct type key = int fun less (a : int, b) = a < b end)
val t = IntTree.insert (2, IntTree.insert (1, IntTree.Leaf))
val n = IntTree.size t
fun leftmost IntTree.Leaf = NONE
  | leftmost (IntTree.Node (IntTree.Leaf, k, _)) = SOME k
  | leftmost (IntTree.Node (l, _, _)) = leftmost l
val m = leftmost t
(* CHECK-STDOUT: val t : IntTree.tree *)
(* CHECK-STDOUT-NEXT: val n : int *)
(* CHECK-STDOUT-NEXT: val leftmost : IntTree.tree -> int option *)
(* CHECK-STDOUT-NEXT: val m : int option *)
(* CHECK-RUN-EXIT: 0 *)
