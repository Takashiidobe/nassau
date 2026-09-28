datatype t = A | B of int
fun f A = 0 | f B = 1
(* CHECK-ERR: × constructor 'B' requires an argument *)
(* CHECK-ERR: :2:17] *)
