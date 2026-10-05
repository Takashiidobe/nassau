(* SML'97 grammar, program: functor opened. *)
fun pi n = print (Int.toString n ^ "\n")
functor F (val x : int type t val f : t -> int) = struct val y = x + 1 end
structure A = F (val x = 1 type t = int fun f a = a) val _ = pi A.y
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
