(* mlton regression/char0.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun f c =
   case c of
      #"a" => ()
    | _ => raise Fail "bug"

val _ = f #"a"
val _ = f #"a"

       
(* CHECK-EXIT: 0 *)
