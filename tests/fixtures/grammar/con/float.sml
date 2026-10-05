(* SML'97 grammar, constant: float. *)
val x : real = 1.5
val y = ~2.25
val _ = if x > 1.0 andalso y < 0.0 then print "ok\n" else print "FAIL\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
