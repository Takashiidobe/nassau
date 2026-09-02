(* mlton regression/fail/modules.32.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:>
   sig
      type t
      val x: t
   end =
   struct
      type t = real
      val x = 13.0
   end
val _ = S.x = S.x
(* CHECK-ERR: × type S.t does not admit equality *)
(* CHECK-ERR: :11:9] *)
