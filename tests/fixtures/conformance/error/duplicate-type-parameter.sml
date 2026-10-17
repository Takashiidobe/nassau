(* nassau-ugc.23 *)
type ('a, 'a) t = 'a
(* CHECK-ERR: × duplicate type variable ''a' *)
(* CHECK-ERR: :2:15] *)
