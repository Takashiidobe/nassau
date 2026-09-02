(* mlton regression/fail/datatype.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
datatype t = A
and u = A
(* CHECK-ERR: × duplicate constructor name 'A' in datatype declaration *)
(* CHECK-ERR: :3:9] *)
