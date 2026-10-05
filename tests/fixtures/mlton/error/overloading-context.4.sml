(* mlton regression/fail/overloading-context.4.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val x = 0.0
;
val _ = x: Real32.real
(* CHECK-ERR: × unbound structure 'Real32' *)
(* CHECK-ERR: :4:12] *)
