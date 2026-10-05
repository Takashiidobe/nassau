(* mlton regression/array4.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun f n =
   if 0 = Array.sub (Array.tabulate (n, fn i => i), 0)
      then ()
   else f 13

val _ = (f 0; raise Fail "bug") handle Subscript => ()
val _ = f 1
(* CHECK-EXIT: 0 *)
