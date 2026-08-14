datatype t = A | B
fun f A = 1 | f B = 2 | f A = 3
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :2:27] *)
