fun add x y = x + y
val z = add 1.0 2.0
(* CHECK-ERR: × expected int, found real *)
(* CHECK-ERR: :2:13] *)
