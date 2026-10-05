(* mlton regression/13.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun plus (a, b) = a
   
functor F () =
   struct
      val _ = plus (plus (1, 2), 3)
   end

infix plus

structure S = F ()
(* CHECK-EXIT: 0 *)
