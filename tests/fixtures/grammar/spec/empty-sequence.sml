(* SML'97 grammar, specification: empty sequence. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig end signature T = sig val x : int ; val y : int ; end
structure A : T = struct val x = 1 val y = 2 end val _ = pi (A.x + A.y)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
