val (a, b) = (1, 2)
val c = a + "b"
(* CHECK-ERR: × expected int, found string *)
(* CHECK-ERR: :2:13] *)
