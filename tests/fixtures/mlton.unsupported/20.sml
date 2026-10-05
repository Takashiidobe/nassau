(* mlton regression/20.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S = struct end;
print "Hello, World!\n";
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: Hello, World! *)
