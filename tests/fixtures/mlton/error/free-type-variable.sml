(* mlton regression/fail/free-type-variable.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
datatype t = T of 'a
(* CHECK-ERR: × unbound type variable 'a in type declaration *)
(* CHECK-ERR: :2:19] *)
