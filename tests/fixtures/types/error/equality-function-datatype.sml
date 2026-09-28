datatype t = F of int -> int
val x = F (fn n => n) = F (fn n => n)
(* CHECK-ERR: × type t does not admit equality *)
(* CHECK-ERR: :2:9] *)
