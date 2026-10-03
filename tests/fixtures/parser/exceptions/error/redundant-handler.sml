exception E of int
val x = (raise E 1) handle _ => 0 | E _ => 1
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :2:37] *)
