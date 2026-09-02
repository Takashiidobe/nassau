(* mlton regression/fail/equal.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val op = = ()

val rec (op =) = fn _ => 13
