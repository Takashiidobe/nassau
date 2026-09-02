(* mlton regression/withtype4.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* withtype4.sml *)

(* Checks scoping rules of withtype *)

type u = real

datatype t = T of u * v
withtype u = int
and      v = u

val x = T(1, 1.0);

fun uEq (a: u, b: u) = a = b
(* CHECK-EXIT: 0 *)
