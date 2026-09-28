datatype t = A of pair withtype pair = int * int
val x : t = A 1
(* CHECK-ERR: × expected int * int, found int *)
(* CHECK-ERR: :2:15] *)
