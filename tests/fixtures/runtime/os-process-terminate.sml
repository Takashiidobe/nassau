val _ = OS.Process.atExit (fn () => print "skipped\n")
val _ = print "terminating\n"
val _ = OS.Process.terminate OS.Process.failure
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: terminating *)
