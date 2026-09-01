(* SML'97 grammar, specification: exception structure. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig exception E of int and F structure M : sig val x : int end end
structure A : S = struct exception E of int exception F structure M = struct val x = 1 end end
val _ = pi ((raise A.E 2) handle A.E n => n + A.M.x)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
