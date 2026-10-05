(* SML'97 grammar, pattern: infix construction. *)
fun pi n = print (Int.toString n ^ "\n")
datatype t = :+: of int * int
infix :+:
fun f (a :+: b) = a + b
fun g (x :: _) = x
  | g [] = 0
val _ = pi (f (op :+: (1, 2)) + g [4])
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 7 *)
