datatype t = A
val a = A
datatype t = B
val b : t = a
(* CHECK-ERR: × expected t, found t/2 *)
(* CHECK-ERR: :4:13] *)
