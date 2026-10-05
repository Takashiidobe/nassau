(* mlton regression/fail/pat.1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ =
   case 13.0 of
      14.0 => ()
;
(* CHECK-ERR: × expected a pattern; real constants cannot be patterns *)
(* CHECK-ERR: :4:7] *)
