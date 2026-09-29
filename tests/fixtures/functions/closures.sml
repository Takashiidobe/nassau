(* Functions returned from functions keep the variables they capture. *)
fun adder n = fn x => x + n
val addFive = adder 5
val addTen = adder 10
fun twice f = fn x => f (f x)
val _ = print (Int.toString (addFive 1) ^ " " ^ Int.toString (addTen 1) ^ "\n")
val _ = print (Int.toString (twice addFive 0) ^ "\n")
val _ = print (Int.toString ((twice (twice addTen)) 0) ^ "\n")
(* Functions are values that can be stored in lists. *)
fun applyAll ([], x) = []
  | applyAll (f :: rest, x) = f x :: applyAll (rest, x)
val results = applyAll ([addFive, addTen, fn x => x * x], 7)
fun sum [] = 0 | sum (x :: rest) = x + sum rest
val _ = print (Int.toString (sum results) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 11 *)
(* CHECK-STDOUT-NEXT: 10 *)
(* CHECK-STDOUT-NEXT: 40 *)
(* CHECK-STDOUT-NEXT: 78 *)
