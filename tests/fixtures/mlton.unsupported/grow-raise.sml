(* mlton regression/grow-raise.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
exception E
val rec loop =
   fn 0 => raise E
    | n => 1 + loop(n - 1)

val _ = loop 1000000 handle E => 13
(* CHECK-EXIT: 0 *)
