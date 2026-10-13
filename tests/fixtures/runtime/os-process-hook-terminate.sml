val _ = OS.Process.atExit (fn () => print "skipped\n")
val _ = OS.Process.atExit (fn () =>
  (print "terminating hook\n";
   OS.Process.terminate OS.Process.failure;
   print "returned from terminate\n"))
val _ = OS.Process.exit OS.Process.success
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: terminating hook *)
