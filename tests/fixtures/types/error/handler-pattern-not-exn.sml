val x = 1 handle 0 => 2
(* CHECK-ERR: × expected exn, found int *)
(* CHECK-ERR: :1:18] *)
