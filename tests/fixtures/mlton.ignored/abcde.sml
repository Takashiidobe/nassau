(* mlton regression/abcde.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML exits with status 1 *)
fun a x = (b x; c x)
and b x = (a x; c x; d x)
and c x = d x
and d x = e x
and e x = d x

val _ = a 13
