(* SML'97 grammar, specification: sharing structure signature. *)
signature T = sig type t end
signature S = sig structure A : T structure B : T sharing A = B end
val _ = print "ok\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
