(* SML'97 grammar, declaration: fixity local scope. *)
fun pi n = print (Int.toString n ^ "\n")
fun f (a, b) = a - b
val _ = pi (let infix f in 5 f 2 end)
val _ = pi (f (5, 2))
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 3 *)
