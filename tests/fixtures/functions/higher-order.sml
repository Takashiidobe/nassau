(* map and foldl written in SML take functions as tupled arguments. *)
fun map (f, []) = []
  | map (f, x :: rest) = f x :: map (f, rest)
fun foldl (f, acc, []) = acc
  | foldl (f, acc, x :: rest) = foldl (f, f (x, acc), rest)
fun show xs = foldl (fn (x, acc) => acc ^ " " ^ Int.toString x, "", xs)
val offset = 10
val shifted = map (fn x => x + offset, [1, 2, 3])
val total = foldl (fn (x, acc) => x + acc, 0, shifted)
val _ = print (show shifted ^ "\n")
val _ = print (Int.toString total ^ "\n")
val _ = print (foldl (fn (s, acc) => acc ^ s, "", map (Int.toString, [4, 5, 6])) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT:  11 12 13 *)
(* CHECK-STDOUT-NEXT: 36 *)
(* CHECK-STDOUT-NEXT: 456 *)
