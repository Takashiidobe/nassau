(* mlton regression/fail/modules.23.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
structure S:
   sig
      val f: 'a -> 'a list
   end =
   struct
      fun f x =
         if x = x
            then []
         else [x]
   end
(* CHECK-ERR: × value f does not match its specification: the structure has type ''a -> *)
(* CHECK-ERR: :6:4] *)
