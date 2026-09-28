exception E of int
val x : exn = E
(* CHECK-ERR: × expected exn, found int -> exn *)
(* CHECK-ERR: :2:15] *)
