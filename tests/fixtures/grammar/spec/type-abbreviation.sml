(* SML'97 grammar, specification: type abbreviation. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig type t = int val x : t end
structure A :> S = struct type t = int val x = 3 end val _ = pi (A.x + 1)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
