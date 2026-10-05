(* mlton regression/fail/3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
type t = 'a * 'b
(* CHECK-ERR: × unbound type variable 'a in type declaration *)
(* CHECK-ERR: :2:10] *)
