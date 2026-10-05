(* mlton regression/fail/exception.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
exception E
and E
(* CHECK-ERR: × duplicate exception name 'E' in exception declaration *)
(* CHECK-ERR: :2:1] *)
