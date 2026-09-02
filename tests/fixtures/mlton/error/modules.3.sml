(* mlton regression/fail/modules.3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F (type t
           datatype u = U of t
           eqtype v
           sharing type t = v) =
   struct
      fun f (u: u) = u = u
   end
(* CHECK-ERR: × type u does not admit equality *)
(* CHECK-ERR: :7:22] *)
