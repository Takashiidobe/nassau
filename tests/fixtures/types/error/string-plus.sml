fun join (a, b) = a + b
val x = join ("a", "b")
(* CHECK-ERR: × expected int * int, found string * string *)
(* CHECK-ERR: :2:14] *)
