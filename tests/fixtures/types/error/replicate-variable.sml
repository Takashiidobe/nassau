val x = 1
exception E = x
(* CHECK-ERR: × 'x' is not an exception *)
(* CHECK-ERR: :2:1] *)
