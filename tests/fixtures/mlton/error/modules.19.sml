(* mlton regression/fail/modules.19.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F (type t
           type u) =
   struct
      val id: t -> u = fn x => x
   end
(* CHECK-ERR: × expected t -> u, found t -> t *)
(* CHECK-ERR: :5:24] *)
