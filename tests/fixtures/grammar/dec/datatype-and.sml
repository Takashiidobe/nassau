(* SML'97 grammar, declaration: datatype and. *)
fun pi n = print (Int.toString n ^ "\n")
datatype a = A of b | Z and b = B of a
fun da Z = 0 | da (A (B x)) = 1 + da x val _ = pi (da (A (B (A (B Z)))))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
