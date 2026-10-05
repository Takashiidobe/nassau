(* mlton regression/23.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML timed out *)
fun f l =
   case l of
      [] => f l
    | _ :: l => f l
   
val _ = f [13]
