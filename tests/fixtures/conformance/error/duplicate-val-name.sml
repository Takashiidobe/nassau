(* nassau-ugc.11 *)
val x = 1 and x = 2
(* CHECK-ERR: × duplicate variable 'x' in pattern *)
(* CHECK-ERR: :2:15] *)
