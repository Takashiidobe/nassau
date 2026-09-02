(* mlton regression/cycle.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* IGNORED: Poly/ML exits with status 1 *)
datatype t = T of u | V
and u = U of t * t

fun f V = T (U (f V,f V))
  | f (T _) = V

val _ = f V
