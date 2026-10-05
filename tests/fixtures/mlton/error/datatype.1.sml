(* mlton regression/fail/datatype.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
datatype t = T of int * t
withtype t = real
(* CHECK-ERR: × duplicate type name 't' in type declaration *)
(* CHECK-ERR: :2:1] *)
