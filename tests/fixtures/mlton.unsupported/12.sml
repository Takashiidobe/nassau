(* mlton regression/12.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun f x = x
val _ = f (0w1: Word8.word)
val _ = f (0w1: Word.word)
(* CHECK-EXIT: 0 *)
