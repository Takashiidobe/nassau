val triple = (1, "two", 3.0)
val () = print (#2 triple ^ "\n")
val person = {name = "Ada", age = 36}
val () = print (#name person ^ " " ^ Int.toString (#age person) ^ "\n")
fun map f [] = [] | map f (x :: xs) = f x :: map f xs
fun concat [] = "" | concat (s :: rest) = s ^ concat rest
val names = map #name [{name = "a", age = 1}, {name = "b", age = 2}]
val () = print (concat names ^ "\n")
val seconds = map #2 [(1, "x"), (2, "y")]
val () = print (concat seconds ^ "\n")
fun firstOf (pair : int * int) = #1 pair
val () = print (Int.toString (firstOf (7, 8) + #2 (7, 8)) ^ "\n")
val get = #age : {name : string, age : int} -> int
val () = print (Int.toString (get person) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: two *)
(* CHECK-STDOUT-NEXT: Ada 36 *)
(* CHECK-STDOUT-NEXT: ab *)
(* CHECK-STDOUT-NEXT: xy *)
(* CHECK-STDOUT-NEXT: 15 *)
(* CHECK-STDOUT-NEXT: 36 *)
