(* SML'97 grammar, type: longid constructor. *)
fun pi n = print (Int.toString n ^ "\n")
structure S = struct type t = int type 'a box = 'a list end
val x : S.t = 3 val y : int S.box = [x] val _ = pi (x + length y)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 4 *)
