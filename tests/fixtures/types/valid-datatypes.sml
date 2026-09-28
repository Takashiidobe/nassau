datatype color = Red | Green | Blue
datatype 'a tree = Leaf | Node of 'a tree * 'a * 'a tree
datatype ('a, 'b) either = Left of 'a | Right of 'b
val c = Red
val t = Node (Leaf, 1, Leaf)
val mixed = [Left 1, Right "s", Left 2]
val leaf = Leaf
val node = Node
val left = Left
fun name Red = "r" | name Green = "g" | name Blue = "b"
fun depth Leaf = 0
  | depth (Node (l, _, r)) =
      let val a = depth l val b = depth r in if a > b then a + 1 else b + 1 end
fun insert (x, Leaf) = Node (Leaf, x, Leaf)
  | insert (x, Node (l, y, r)) =
      if x < y then Node (insert (x, l), y, r) else Node (l, y, insert (x, r))
fun to_list Leaf = [] | to_list (Node (l, x, r)) = to_list l @ [x] @ to_list r
fun swap (Left a) = Right a | swap (Right b) = Left b
val sorted = to_list (insert (2, insert (3, insert (1, Leaf))))
(* CHECK-STDOUT: val c : color *)
(* CHECK-STDOUT-NEXT: val t : int tree *)
(* CHECK-STDOUT-NEXT: val mixed : (int,string) either list *)
(* CHECK-STDOUT-NEXT: val leaf : 'a tree *)
(* CHECK-STDOUT-NEXT: val node : 'a tree * 'a * 'a tree -> 'a tree *)
(* CHECK-STDOUT-NEXT: val left : 'a -> ('a,'b) either *)
(* CHECK-STDOUT-NEXT: val name : color -> string *)
(* CHECK-STDOUT-NEXT: val depth : 'a tree -> int *)
(* CHECK-STDOUT-NEXT: val insert : int * int tree -> int tree *)
(* CHECK-STDOUT-NEXT: val to_list : 'a tree -> 'a list *)
(* CHECK-STDOUT-NEXT: val swap : ('a,'b) either -> ('b,'a) either *)
(* CHECK-STDOUT-NEXT: val sorted : int list *)
