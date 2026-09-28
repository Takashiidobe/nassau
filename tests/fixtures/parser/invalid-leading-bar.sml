val x = case 1 of | 0 => 1 | _ => 2
(* CHECK-ERR: × expected a pattern *)
(* CHECK-ERR: :1:19] *)
