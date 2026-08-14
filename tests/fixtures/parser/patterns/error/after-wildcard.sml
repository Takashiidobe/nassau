val x = fn _ => 1 | 0 => 2
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:21] *)
