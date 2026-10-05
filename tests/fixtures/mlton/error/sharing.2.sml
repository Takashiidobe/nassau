(* mlton regression/fail/sharing.2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F (structure A: sig type t end
           structure B: sig end
           structure C: sig type t end
           sharing A = B
           sharing B = C) =
   struct
      val _: A.t -> C.t = fn x => x
   end
(* CHECK-ERR: × expected ?.t -> ?.t, found ?.t -> ?.t *)
(* CHECK-ERR: :8:27] *)
