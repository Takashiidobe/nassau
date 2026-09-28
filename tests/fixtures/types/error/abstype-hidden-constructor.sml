abstype t = A of int with fun make n = A n end
val x = A 1
(* CHECK-ERR: × unbound variable 'A' *)
(* CHECK-ERR: :2:9] *)
