exception E of int
val x = (raise E 1) handle E s => size s
(* CHECK-ERR: × expected string, found int *)
(* CHECK-ERR: :2:40] *)
