(* mlton regression/fail/5.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
datatype t = A | =
(* CHECK-ERR: × expected a constructor name *)
(* CHECK-ERR: :2:18] *)
