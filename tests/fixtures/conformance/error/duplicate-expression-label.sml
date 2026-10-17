(* nassau-ugc.23 *)
val x = {a=1, a=2}
(* CHECK-ERR: × duplicate record label 'a' *)
(* CHECK-ERR: :2:18] *)
