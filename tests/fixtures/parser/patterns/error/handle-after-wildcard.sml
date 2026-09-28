val x = (1 div 0) handle _ => 1 | _ => 2
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:35] *)
