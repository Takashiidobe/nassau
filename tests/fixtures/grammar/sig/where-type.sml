(* SML'97 grammar, signature expression: where type. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig type t type u val x : t end
structure A :> S where type t = int and type u = bool = struct type t = int type u = bool val x = 5 end
val _ = pi (A.x + 1)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 *)
