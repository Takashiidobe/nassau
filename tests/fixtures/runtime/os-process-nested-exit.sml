val _ = OS.Process.atExit (fn () => print "oldest\n")
val _ = OS.Process.atExit (fn () => raise Fail "ignored")
val _ = OS.Process.atExit (fn () =>
  (print "nested exit\n";
   OS.Process.atExit (fn () => print "new hook\n");
   OS.Process.exit OS.Process.failure;
   print "returned from exit\n"))
val _ = OS.Process.atExit (fn () => print "newest\n")
val _ = OS.Process.exit OS.Process.success
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: newest *)
(* CHECK-STDOUT-NEXT: nested exit *)
(* CHECK-STDOUT-NEXT: oldest *)
