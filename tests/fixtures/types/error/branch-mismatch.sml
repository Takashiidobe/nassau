datatype a = A
datatype b = B
val x = if true then A else B
(* CHECK-ERR: × expected a, found b *)
(* CHECK-ERR: :3:29] *)
