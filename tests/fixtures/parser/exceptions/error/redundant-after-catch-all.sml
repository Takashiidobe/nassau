exception E
val x = (raise E) handle _ => 0 | E => 1
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :2:35] *)
