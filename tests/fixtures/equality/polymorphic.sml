(* Equality on an equality type variable calls the runtime. *)
fun member (x, []) = false
  | member (x, y :: ys) = x = y orelse member (x, ys)
fun count (x, xs) = let fun go ([], n) = n | go (y :: ys, n) = go (ys, if x = y then n + 1 else n) in go (xs, 0) end
datatype suit = Hearts | Spades
fun say b = print ((if b then "yes" else "no") ^ "\n")
val () = say (member (3, [1, 2, 3]))
val () = say (member ("d", ["a", "b"]))
val () = say (member (Spades, [Hearts, Spades]))
val () = say (member ([1, 2], [[1], [1, 2]]))
val () = say (member (SOME (1, "a"), [NONE, SOME (1, "b")]))
val () = print (Int.toString (count (Hearts, [Hearts, Spades, Hearts])) ^ "\n")
fun dedupe [] = []
  | dedupe (x :: xs) = if member (x, xs) then dedupe xs else x :: dedupe xs
val () = print (Int.toString (count (1, dedupe [1, 1, 2, 1, 3])) ^ "\n")
(* op = and op <> are functions too. *)
fun all eq (x :: y :: rest) = eq (x, y) andalso all eq (y :: rest) | all eq _ = true
val () = say (all (op =) [Spades, Spades] andalso not (all (op <>) [1, 2, 2]))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: yes *)
(* CHECK-STDOUT-NEXT: no *)
(* CHECK-STDOUT-NEXT: yes *)
(* CHECK-STDOUT-NEXT: yes *)
(* CHECK-STDOUT-NEXT: no *)
(* CHECK-STDOUT-NEXT: 2 *)
(* CHECK-STDOUT-NEXT: 1 *)
(* CHECK-STDOUT-NEXT: yes *)
