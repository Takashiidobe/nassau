(* mlton regression/vector2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
open Vector
val v = tabulate(13, fn i => fn j => i + j)
val _ = print(Int.toString(sub(v, 5) 1))
val _ = print "\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 6 *)
