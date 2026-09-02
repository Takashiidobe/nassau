(* mlton regression/big-array.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
open Array

val a'' = tabulate (1000000, fn i => i)
val _ = sub (a'', 0) + sub (a'', 1)
val _ = print "OK\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: OK *)
