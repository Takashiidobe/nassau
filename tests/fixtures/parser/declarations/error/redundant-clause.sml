fun f _ = 1 | f 0 = 2
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:17] *)
