(* mlton regression/fail/modules.29.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* Generativity of functors. *)
functor F () =
   struct
      datatype t = T
   end
structure S1 = F ()
structure S2 = F ()
val _ = S1.T = S2.T
(* CHECK-ERR: × expected S1.t, found S2.t *)
(* CHECK-ERR: :9:16] *)
