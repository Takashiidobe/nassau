(* mlton regression/exn2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun die(): 'a =
   let exception Die
   in raise Die
   end

val _ =
   let val _: string = die()
      val _: real = die()
      val _: bool = die()
   in ()
   end handle _ => ()
(* CHECK-EXIT: 0 *)
