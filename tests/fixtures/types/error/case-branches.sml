val x = case 1 of 0 => "zero" | _ => 1
(* CHECK-ERR: × expected string, found int *)
(* CHECK-ERR: :1:38] *)
