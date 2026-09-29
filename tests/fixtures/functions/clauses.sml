(* Clauses over constants, tuples and list patterns. *)
fun sum [] = 0
  | sum (x :: rest) = x + sum rest
fun describe [] = "empty"
  | describe [_] = "one"
  | describe [_, _] = "two"
  | describe _ = "many"
fun max (a, b) = if a > b then a else b
fun greet "world" = "hello, world"
  | greet name = "hi, " ^ name
val _ = print (Int.toString (sum [1, 2, 3, 4]) ^ "\n")
val _ = print (describe [] ^ " " ^ describe [1] ^ " " ^ describe [1, 2] ^ " " ^ describe [1, 2, 3] ^ "\n")
val _ = print (Int.toString (max (3, ~7)) ^ "\n")
val _ = print (greet "world" ^ "\n" ^ greet "you" ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 10 *)
(* CHECK-STDOUT-NEXT: empty one two many *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: hello, world *)
(* CHECK-STDOUT-NEXT: hi, you *)
