(* mlton regression/comment-end.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val ** = 13
val x = ( **)
val _ = 1 + ** + x
(* CHECK-EXIT: 0 *)
