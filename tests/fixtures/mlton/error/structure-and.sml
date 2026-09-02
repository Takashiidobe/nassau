(* mlton regression/fail/structure-and.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S = struct end
and S' = S
(* CHECK-ERR: × unbound structure 'S' *)
(* CHECK-ERR: :3:10] *)
