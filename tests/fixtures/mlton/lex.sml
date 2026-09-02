(* mlton regression/lex.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)

  fun token0(tokFn) = tokFn

  and token1(tokFn, value) = tokFn(value)

  val a = token1(fn _ => "1", 2)
(* CHECK-EXIT: 0 *)
