datatype t = A | B of int
fun f (A x) = x | f (B y) = y
(* CHECK-ERR: × constructor 'A' does not take an argument *)
(* CHECK-ERR: :2:8] *)
