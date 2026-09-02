(* mlton regression/fail/functor-generative-equality.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F () =
   struct
      datatype t = T of int -> int
   end

functor G () =
   struct
      structure S = F ()

      fun f (x: S.t) = x = x
   end
(* CHECK-ERR: × type S.t does not admit equality *)
(* CHECK-ERR: :11:24] *)
