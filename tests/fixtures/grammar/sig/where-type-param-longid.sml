(* SML'97 grammar, signature expression: where type param longid. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig structure M : sig type 'a t end val x : int M.t end
structure A :> S where type 'a M.t = 'a list = struct structure M = struct type 'a t = 'a list end val x = [1, 2] end
val _ = pi (length A.x)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
