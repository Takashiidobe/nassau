val x = fn (_, _) => 1 | (1, 2) => 2
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:26] *)
