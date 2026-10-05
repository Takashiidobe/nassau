(* mlton regression/2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML exits with status 1 *)
datatype t = A | B

val f =
   fn A => 1
    | B => 2

val _ = f(raise Overflow)
val _ = f(raise Bind)
