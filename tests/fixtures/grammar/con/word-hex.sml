(* SML'97 grammar, constant: word hex. *)
val _ = if 0wx2A = 0w42 andalso 0wxff = 0w255 then print "ok\n" else print "FAIL\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
