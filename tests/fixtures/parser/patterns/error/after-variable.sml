val x = case 1 of y => 1 | 2 => 2
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:28] *)
