(* mlton regression/fail/4.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
datatype foo = Foo of 'a
(* CHECK-ERR: × unbound type variable 'a in type declaration *)
(* CHECK-ERR: :2:23] *)
