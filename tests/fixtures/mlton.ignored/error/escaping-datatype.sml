(* mlton regression/fail/escaping-datatype.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
val _ =
   let
      datatype t = T
   in
      (T, fn T => 1)
   end
