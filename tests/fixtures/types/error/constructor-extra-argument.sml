datatype t = A
val x = A 1
(* CHECK-ERR: × cannot apply a value of type t, which is not a function *)
(* CHECK-ERR: :2:9] *)
