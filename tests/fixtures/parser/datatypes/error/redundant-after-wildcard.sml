datatype t = A | B
val f = fn _ => 0 | A => 1
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :2:21] *)
