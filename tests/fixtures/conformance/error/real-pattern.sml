(* nassau-ugc.11 *)
fun f 1.0 = 1 | f _ = 0
(* CHECK-ERR: × expected at least one function parameter *)
(* CHECK-ERR: :2:7] *)
