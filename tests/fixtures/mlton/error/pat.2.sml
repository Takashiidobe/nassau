(* mlton regression/fail/pat.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val {x, x} = {x = 13}
(* CHECK-ERR: × duplicate variable 'x' in pattern *)
(* CHECK-ERR: :2:9] *)
