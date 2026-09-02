(* mlton regression/fail/1.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
functor F () =
   struct
      val x = y
   end

val y = 13

structure S = F ()
(* CHECK-ERR: × unbound variable 'y' *)
(* CHECK-ERR: :4:15] *)
