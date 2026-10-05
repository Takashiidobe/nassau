(* mlton regression/only-one-exception.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML timed out *)
fun f n =
   if n = 0
      then 0
   else f (f (n - 1))

val _ = f 13

fun loop () = loop ()

val _ = loop ()
