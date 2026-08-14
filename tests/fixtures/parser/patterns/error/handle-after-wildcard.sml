val x = (1 div 0) handle _ => 1 | _ => 2
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:35] *)
