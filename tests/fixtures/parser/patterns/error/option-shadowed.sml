val x = fn SOME _ => 1 | SOME 2 => 2 | NONE => 3
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:26] *)
