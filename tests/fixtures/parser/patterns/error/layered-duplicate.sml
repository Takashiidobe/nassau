val x = fn (a as (b, c)) => 1 | (d, e) => 2
(* POLYML-WARNING: redundant *)
(* CHECK-ERR: × match redundant *)
(* CHECK-ERR: :1:33] *)
