(* mlton regression/fail/exp.6.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val rec nil = fn () => ()
(* CHECK-ERR: × expected 'a list, found unit -> unit *)
(* CHECK-ERR: :2:15] *)
