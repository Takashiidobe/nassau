exception E
val x = E 1
(* CHECK-ERR: × cannot apply a value of type exn, which is not a function *)
(* CHECK-ERR: :2:9] *)
