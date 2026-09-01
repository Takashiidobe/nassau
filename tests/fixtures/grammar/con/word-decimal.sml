(* SML'97 grammar, constant: word decimal. *)
val _ = if 0w42 = 0w42 andalso 0w1 <> 0w2 then print "ok\n" else print "FAIL\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
