(* mlton regression/fail/modules.47.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature S =
   sig
      val true: bool
   end
(* CHECK-ERR: × expected a value name *)
(* CHECK-ERR: :4:11] *)
