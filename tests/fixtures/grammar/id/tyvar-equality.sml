(* SML'97 grammar, identifier: tyvar equality. *)
fun eq (x : ''a, y : ''a) = x = y
val _ = if eq (1, 1) andalso not (eq ("a", "b")) then print "ok\n" else print "FAIL\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: ok *)
