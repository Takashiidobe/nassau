datatype t = A | B
fun f A = 1 | f B = 2 | f A = 3
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :2:27] *)
