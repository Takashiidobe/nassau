val x = (fn ref n => n) 5
(* CHECK-ERR: × expected 'a ref, found int *)
(* CHECK-ERR: :1:25] *)
