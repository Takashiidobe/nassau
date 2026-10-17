(* nassau-ugc.11 *)
datatype t = A | A
(* CHECK-ERR: × duplicate constructor name 'A' in datatype declaration *)
(* CHECK-ERR: :2:18] *)
