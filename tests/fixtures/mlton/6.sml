(* mlton regression/6.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun f x = x

val r: (int -> int) ref = ref f

val _ = r := (fn y => y)

val _ = !r 13 + 1
(* CHECK-EXIT: 0 *)
