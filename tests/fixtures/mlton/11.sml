(* mlton regression/11.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ = let val x = SOME [] in (valOf x = [1], valOf x = [true]) end
(* CHECK-EXIT: 0 *)
