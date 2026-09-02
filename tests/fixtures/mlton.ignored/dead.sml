(* mlton regression/dead.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML timed out *)
datatype t = A of t

fun f (A y) = f y

fun g () = A (g ())

val _ = f (g ())
