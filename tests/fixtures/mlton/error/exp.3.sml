(* mlton regression/fail/exp.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun f x x = 13
(* CHECK-ERR: × duplicate variable 'x' in pattern *)
(* CHECK-ERR: :2:9] *)
