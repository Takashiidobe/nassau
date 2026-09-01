(* SML'97 grammar, declaration: local. *)
fun pi n = print (Int.toString n ^ "\n")
local val h = 2 in val x = h * 3 end val _ = pi x
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 *)
