datatype t = A of int
val x : t = A
(* CHECK-ERR: × expected t, found int -> t *)
(* CHECK-ERR: :2:13] *)
