(* mlton regression/fail/modules.30.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:> sig type t end = struct type t = unit end
val _ = (): S.t
(* CHECK-ERR: × expected S.t, found unit *)
(* CHECK-ERR: :3:9] *)
