exception E of int
val x = (raise E 1) handle E _ => 0 | E _ => 1
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :2:39] *)
