(* SML'97 grammar, structure expression: annotations. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig type t val x : t val get : t -> int end
structure A = struct type t = int val x = 1 fun get y = y end
structure T = A : S structure O = A :> S
val _ = pi (T.x + 1) val _ = pi (O.get O.x)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
(* CHECK-STDOUT-NEXT: 1 *)
