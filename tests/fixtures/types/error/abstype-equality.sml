abstype t = A of int with val a = A 1 end
val x = a = a
(* CHECK-ERR: × type t does not admit equality *)
(* CHECK-ERR: :2:9] *)
