(* mlton regression/fail/modules.35.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F (ZZZ: sig end) = struct end
structure Z = ZZZ
(* CHECK-ERR: × unbound structure 'ZZZ' *)
(* CHECK-ERR: :3:15] *)
