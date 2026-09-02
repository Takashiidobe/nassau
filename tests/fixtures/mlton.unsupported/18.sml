(* mlton regression/18.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
print "Hello,";
val _ = print " ";
print "World!\n";
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: Hello, World! *)
