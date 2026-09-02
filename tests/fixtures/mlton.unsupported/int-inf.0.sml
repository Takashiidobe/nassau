(* mlton regression/int-inf.0.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ = print (concat [IntInf.toString 0x80000000, "\n"])
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2147483648 *)
