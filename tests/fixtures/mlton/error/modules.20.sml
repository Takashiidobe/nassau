(* mlton regression/fail/modules.20.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F (type t) = struct fun f (x: t) = x = x end
(* CHECK-ERR: × type t does not admit equality *)
(* CHECK-ERR: :2:44] *)
