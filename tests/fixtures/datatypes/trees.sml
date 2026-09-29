datatype 'a tree = Leaf | Node of 'a tree * 'a * 'a tree
fun insert (x, Leaf) = Node (Leaf, x, Leaf)
  | insert (x, t as Node (l, y, r)) =
      if x < y then Node (insert (x, l), y, r)
      else if x > y then Node (l, y, insert (x, r))
      else t
fun toList Leaf = []
  | toList (Node (l, x, r)) = toList l @ [x] @ toList r
and op @ ([], ys) = ys
  | op @ (x :: xs, ys) = x :: (xs @ ys)
fun build [] = Leaf
  | build (x :: xs) = insert (x, build xs)
fun depth Leaf = 0
  | depth (Node (l, _, r)) = let val a = depth l val b = depth r in 1 + (if a > b then a else b) end
fun join [] = "" | join [x] = Int.toString x | join (x :: xs) = Int.toString x ^ " " ^ join xs
val t = build [5, 3, 8, 1, 4, 7, 9, 3]
val () = print (join (toList t) ^ "\n")
val () = print (Int.toString (depth t) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 3 4 5 7 8 9 *)
(* CHECK-STDOUT-NEXT: 5 *)
