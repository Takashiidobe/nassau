(* mlton regression/fail/type.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
type t = u

type 'a t = unit
type u = t

datatype 'a t = T
type u = t
(* CHECK-ERR: × unbound type 'u' *)
(* CHECK-ERR: :2:10] *)
