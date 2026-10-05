(* SML'97 grammar, expression: andalso orelse. *)
val _ = if true andalso (false orelse true) andalso not (false andalso true) then print "ok\n" else print "FAIL\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
