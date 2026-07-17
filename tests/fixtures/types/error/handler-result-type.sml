exception E
val x = (raise E) handle E => "a" | _ => 1
(* CHECK-ERR: × expected string, found int *)
(* CHECK-ERR: :2:42] *)
