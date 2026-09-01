(* SML'97 grammar, program: sequence semicolons. *)
fun pi n = print (Int.toString n ^ "\n")
signature A = sig end; structure M = struct end; functor F () = struct end; val _ = pi 1;
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
