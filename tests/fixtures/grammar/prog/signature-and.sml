(* SML'97 grammar, program: signature and. *)
fun pi n = print (Int.toString n ^ "\n")
signature A = sig val x : int end and B = sig val y : int end
structure M : A = struct val x = 1 end structure N : B = struct val y = 2 end val _ = pi (M.x + N.y)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
