(* mlton regression/19.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
print "Hello,";
val _ = print " ";
print "World!";
val _ = print "\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: Hello, World! *)
