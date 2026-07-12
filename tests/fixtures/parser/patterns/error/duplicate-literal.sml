val x = fn 1 => 1 | 1 => 2 | _ => 3
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:21] *)
