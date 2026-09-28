val x = fn _ => 1 | 0 => 2
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:21] *)
