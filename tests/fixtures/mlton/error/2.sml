(* mlton regression/fail/2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ =
   let val x = ref []
   in (!x = [1], !x = [true])
   end
(* CHECK-ERR: × expected int list, found bool list *)
(* CHECK-ERR: :4:23] *)
