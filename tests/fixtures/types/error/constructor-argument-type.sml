datatype t = A of int
val x = A "one"
(* CHECK-ERR: × expected int, found string *)
(* CHECK-ERR: :2:11] *)
