(* mlton regression/fail/signature-and.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
signature SIG = sig end
and SIG' = SIG
(* CHECK-ERR: × unbound signature 'SIG' *)
(* CHECK-ERR: :3:12] *)
