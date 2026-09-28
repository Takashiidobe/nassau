fun f 0 = 1 | g 1 = 2
(* CHECK-ERR: × expected every clause to define the same function *)
(* CHECK-ERR: :1:21] *)
