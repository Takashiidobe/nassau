(* SML'97 grammar, expression: let sequence. *)
val _ = let val x = "x" in print x; print "y"; print "\n" end
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: xy *)
