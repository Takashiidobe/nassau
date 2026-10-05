(* SML'97 grammar, signature expression: where type multi. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig type t type u val x : t val y : u end
structure A :> S where type t = int where type u = int = struct type t = int type u = int val x = 1 val y = 2 end
val _ = pi (A.x + A.y)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
