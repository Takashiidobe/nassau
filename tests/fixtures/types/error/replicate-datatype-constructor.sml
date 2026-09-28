datatype t = T
exception E = T
(* CHECK-ERR: × 'T' is not an exception *)
(* CHECK-ERR: :2:1] *)
