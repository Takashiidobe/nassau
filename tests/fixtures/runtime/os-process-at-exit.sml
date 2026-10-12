val _ = OS.Process.atExit (fn () => (print "a\n"; OS.Process.atExit (fn () => print "nested\n")))
val _ = OS.Process.atExit (fn () => raise Fail "boom")
val _ = print "end\n"
val _ = OS.Process.atExit (fn () => print "last registered\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: end *)
(* CHECK-STDOUT-NEXT: last registered *)
(* CHECK-STDOUT-NEXT: a *)
