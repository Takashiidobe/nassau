(* mlton regression/withtype.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* withtype.sml *)

(* Checks scoping rules of withtype *)

type u = int

datatype t = T of u * v
withtype u = bool
and      v = u

val x = T(true, 6);
(* CHECK-EXIT: 0 *)
