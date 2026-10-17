(* nassau-ugc.11 *)
val {a=x, a=y} = {a=1}
(* CHECK-ERR: × duplicate record label 'a' *)
(* CHECK-ERR: :2:16] *)
