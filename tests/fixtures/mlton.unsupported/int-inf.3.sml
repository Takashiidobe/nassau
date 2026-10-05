(* mlton regression/int-inf.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ = print (Bool.toString ((1: IntInf.int) < 2))
val _ = print (IntInf.toString (IntInf.quot (1234567890, 100000)))
val _ = print "\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: true12345 *)
