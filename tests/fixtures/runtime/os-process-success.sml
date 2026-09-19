val () = print "start\n"
val _ = OS.Process.exit OS.Process.success
val () = print "unreachable\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: start *)
