exception E
val x = E = E
(* CHECK-ERR: × type exn does not admit equality *)
(* CHECK-ERR: :2:9] *)
