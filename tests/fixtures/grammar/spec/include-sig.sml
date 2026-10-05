(* SML'97 grammar, specification: include sig. *)
fun pi n = print (Int.toString n ^ "\n")
signature A = sig val x : int end
signature B = sig include A val y : int end
signature C = sig include sig val z : int end end
structure M : B = struct val x = 1 val y = 2 end
structure N : C = struct val z = 3 end
val _ = pi (M.x + M.y + N.z)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 *)
