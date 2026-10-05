(* mlton regression/fail/datatype.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
datatype t = A | A
(* CHECK-ERR: × duplicate constructor name 'A' in datatype declaration *)
(* CHECK-ERR: :2:18] *)
