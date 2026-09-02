(* mlton regression/vector3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val v = Vector.tabulate (1000, fn i => ())
val r = ref 0
val _ = r := Vector.length v
val _ = print (concat [Int.toString (!r), "\n"])
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1000 *)
