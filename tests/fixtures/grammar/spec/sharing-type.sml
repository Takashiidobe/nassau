(* SML'97 grammar, specification: sharing type. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig type t type u sharing type t = u val f : t -> u end
structure A : S = struct type t = int type u = int fun f x = x end val _ = pi (A.f 3)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
