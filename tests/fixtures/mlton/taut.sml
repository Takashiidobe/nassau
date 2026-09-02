(* mlton regression/taut.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* a tautology checker *)

datatype t =
   Base of bool
 | Fun of bool -> t

val rec taut =
   fn Base b => b
    | Fun f => taut (f true) andalso taut (f false)

val rec bigTrue =
   fn 0 => Base true
    | n => Fun (fn _ => bigTrue (n - 1))

val _ = taut (bigTrue 12)
   
(* CHECK-EXIT: 0 *)
