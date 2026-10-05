(* mlton regression/fail/modules.31.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:>
   sig
      type t
      val x: t
   end =
   struct
      type t = unit
      val x = ()
   end
val _ = S.x = S.x
(* CHECK-ERR: × type S.t does not admit equality *)
(* CHECK-ERR: :11:9] *)
