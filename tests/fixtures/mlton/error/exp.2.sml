(* mlton regression/fail/exp.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val x = 13
and x = ();
(* CHECK-ERR: × duplicate variable 'x' in pattern *)
(* CHECK-ERR: :3:5] *)
