fun f 0 = 1 | f x y = 2
(* CHECK-ERR: × expected every clause to have the same number of parameters *)
(* CHECK-ERR: :1:23] *)
