val r = ref 0
val x = r + 1
(* CHECK-ERR: × operator is not defined for type int ref *)
(* CHECK-ERR: :2:9] *)
