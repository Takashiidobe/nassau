(* SML'97 grammar, constant: char. *)
val _ = if #"a" = #"a" andalso #"\n" <> #"a" andalso #"\065" = #"A" then print "ok\n" else print "FAIL\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
