(* nassau-ugc.24 *)
datatype t = C
val C as x = C
(* CHECK-ERR: × 'C' cannot be used as a binding name *)
(* CHECK-ERR: :3:5] *)
