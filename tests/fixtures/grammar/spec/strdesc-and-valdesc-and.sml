(* SML'97 grammar, specification: strdesc and valdesc and. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig structure A : sig val x : int end and B : sig val y : int end val p : int and q : int end
structure M : S = struct structure A = struct val x = 1 end structure B = struct val y = 2 end val p = 3 val q = 4 end
val _ = pi (M.A.x + M.B.y + M.p + M.q)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 10 *)
