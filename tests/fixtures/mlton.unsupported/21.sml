(* mlton regression/21.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val 'a rec f = fn x => x
(* CHECK-EXIT: 0 *)
