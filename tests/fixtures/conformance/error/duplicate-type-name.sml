(* nassau-ugc.11 *)
type t = int and t = bool
(* CHECK-ERR: × duplicate type name 't' in type declaration *)
(* CHECK-ERR: :2:1] *)
