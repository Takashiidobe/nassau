(* SML'97 grammar, declaration: fixity. *)
fun pi n = print (Int.toString n ^ "\n")
infix 6 ++ infixr 7 ** infix *** 
fun a ++ b = a + b fun a ** b = a * b fun a *** b = a - b
val _ = pi (1 ++ 2 ** 3)
val _ = pi (10 *** 3 *** 2)
nonfix ++ val _ = pi (++ (1, 1))
infixr 5 --- fun a --- b = a - b val _ = pi (10 --- 3 --- 2)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 7 *)
(* CHECK-STDOUT-NEXT: 5 *)
(* CHECK-STDOUT-NEXT: 2 *)
(* CHECK-STDOUT-NEXT: 9 *)
