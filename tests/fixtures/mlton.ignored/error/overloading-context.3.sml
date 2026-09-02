(* mlton regression/fail/overloading-context.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
val x = 0w0
signature S = sig end
val _ = x: Word8.word
