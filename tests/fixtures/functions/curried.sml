(* fun f x y is a function returning a function. *)
fun add x y = x + y
fun volume x y z = x * y * z
val increment = add 1
val area = volume 1
val twoByThree = area 2 3
val _ = print (Int.toString (add 2 3) ^ " " ^ Int.toString (increment 41) ^ "\n")
val _ = print (Int.toString (volume 2 3 4) ^ " " ^ Int.toString twoByThree ^ " " ^ Int.toString (volume 1 2 5) ^ "\n")
(* Partial applications are values like any other. *)
fun map f [] = []
  | map f (x :: rest) = f x :: map f rest
fun join [] = ""
  | join [x] = x
  | join (x :: rest) = x ^ "," ^ join rest
val _ = print (join (map Int.toString (map (add 10) [1, 2, 3])) ^ "\n")
(* A curried function whose clauses match on several arguments. *)
fun zip [] _ = []
  | zip _ [] = []
  | zip (x :: xs) (y :: ys) = (x, y) :: zip xs ys
fun sumPairs [] = 0
  | sumPairs ((a, b) :: rest) = a * b + sumPairs rest
val _ = print (Int.toString (sumPairs (zip [1, 2, 3] [4, 5, 6, 7])) ^ "\n")
(* Extra arguments apply the function a saturated call returns. *)
fun choose true = (fn x => fn y => x) | choose false = (fn x => fn y => y)
val _ = print (choose false "left" "right" ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 5 42 *)
(* CHECK-STDOUT-NEXT: 24 6 10 *)
(* CHECK-STDOUT-NEXT: 11,12,13 *)
(* CHECK-STDOUT-NEXT: 32 *)
(* CHECK-STDOUT-NEXT: right *)
