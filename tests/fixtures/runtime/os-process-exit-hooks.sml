val _ = OS.Process.atExit (fn () => print "hook one\n")
val _ = OS.Process.atExit (fn () => print "hook two\n")
val _ = print "exiting\n"
val _ = OS.Process.exit OS.Process.failure
val _ = print "unreachable\n"
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: exiting *)
(* CHECK-STDOUT-NEXT: hook two *)
(* CHECK-STDOUT-NEXT: hook one *)
