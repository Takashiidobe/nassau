val r = ref 0
val x = (r := )
(* CHECK-ERR: × expected an expression *)
(* CHECK-ERR: :2:15] *)
