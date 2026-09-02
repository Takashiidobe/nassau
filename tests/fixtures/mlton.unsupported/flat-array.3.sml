(* mlton regression/flat-array.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val a = Array.tabulate (100, fn i => (i, real i))

val _ = Array.update (a, 0, (1, 100.0))

val i = #1 (Array.sub (a, 0)) + #1 (Array.sub (a, 1))

val x = #2 (Array.sub (a, 0)) + #2 (Array.sub (a, 1))

val () = print (concat [Int.toString i, " ", Real.toString x, "\n"])
   
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 101.0 *)
