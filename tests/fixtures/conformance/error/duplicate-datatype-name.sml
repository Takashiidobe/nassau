(* nassau-ugc.11 *)
datatype t = A and t = B
(* CHECK-ERR: × duplicate type name 't' in type declaration *)
(* CHECK-ERR: :2:1] *)
