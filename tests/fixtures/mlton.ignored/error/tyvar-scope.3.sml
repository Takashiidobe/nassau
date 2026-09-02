(* mlton regression/fail/tyvar-scope.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
val _ =
   fn () =>
   let
      exception E of 'a
      val 'a f = fn z => z
   in
      ()
   end
