(* SML'97 grammar, constant: float scientific. *)
val _ = if 1e3 > 999.0 andalso 1.5e~2 < 1.0 andalso ~2.0E2 < 0.0 andalso 2E1 > 19.0 then print "ok\n" else print "FAIL\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
