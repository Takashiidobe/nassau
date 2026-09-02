(* mlton regression/fail/type.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
type t = int
and t = real
(* CHECK-ERR: × duplicate type name 't' in type declaration *)
(* CHECK-ERR: :2:1] *)
