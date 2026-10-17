type empty = {}
val u : empty = {}
fun f (x : {}) : unit = x
val _ = print (Bool.toString (f u = ()) ^ "\n")
structure S : sig val u : {} end = struct val u = () end
val _ = print (Bool.toString (S.u = u) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: true *)
(* CHECK-STDOUT-NEXT: true *)
