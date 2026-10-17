(* nassau-ugc.11 *)
val {a=x, a=y} = {a=1}
(* CHECK-ERR: × expected {a:'a, a:'b}, found {a:int} *)
(* CHECK-ERR: :2:18] *)
