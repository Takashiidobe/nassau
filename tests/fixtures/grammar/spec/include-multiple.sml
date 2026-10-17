(* SML'97 grammar, specification: include multiple (nassau-ugc.16). *)
fun pi n = print (Int.toString n ^ "\n")
signature A = sig val x : int end
signature D = sig type t end
signature E = sig include A D end
structure O : E = struct val x = 4 type t = int end
val _ = pi O.x
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
