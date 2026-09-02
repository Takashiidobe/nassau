(* mlton regression/9.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun 'a f (x: 'a): 'a = x

val y: int -> int = f
(* CHECK-EXIT: 0 *)
