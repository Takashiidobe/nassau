exception E of string
val x = raise "E"
(* CHECK-ERR: × expected exn, found string *)
(* CHECK-ERR: :2:15] *)
