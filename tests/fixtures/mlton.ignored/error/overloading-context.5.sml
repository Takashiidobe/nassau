(* mlton regression/fail/overloading-context.5.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML accepts this program, which the suite expects to be rejected *)
functor F () =
   struct
      val x = 0w0
      structure S = struct end
      val _ = x: Word8.word
   end

