(* mlton regression/fail/functor-and.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F () = struct end
and F' () = F ()
(* CHECK-ERR: × unbound functor 'F' *)
(* CHECK-ERR: :3:13] *)
