val x = case true of true => 1 | false => 2 | _ => 3
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:47] *)
