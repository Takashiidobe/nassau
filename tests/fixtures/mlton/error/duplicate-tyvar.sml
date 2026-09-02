(* mlton regression/fail/duplicate-tyvar.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
type ('a, 'a) t = unit

datatype ('a, 'a) t = T of ('a, 'a) u
withtype ('b, 'b) u = 'b * 'b

fun ('a, 'a) id (x: 'a) : 'a = x
val ('a, 'a) id : 'a -> 'a = fn (x: 'a) => x : 'a
(* CHECK-ERR: × expected a pattern *)
(* CHECK-ERR: :7:6] *)
