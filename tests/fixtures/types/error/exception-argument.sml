exception E of int
val x = E "one"
(* CHECK-ERR: × expected int, found string *)
(* CHECK-ERR: :2:11] *)
