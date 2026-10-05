(* mlton regression/fail/tyvar-scope.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
fun f (x: 'a) =
   let
      fun 'a g (y: 'a) = y
   in
      ()
   end
