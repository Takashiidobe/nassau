(* mlton regression/lambda-list-ref.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val r : (int -> int) list ref = ref []

val _ = r := (fn x => x + 1) :: ! r
val _ = r := (fn x => x + 2) :: ! r

val _ = app (fn f => (f 13; ())) (!r)
                                     
   
(* CHECK-EXIT: 0 *)
