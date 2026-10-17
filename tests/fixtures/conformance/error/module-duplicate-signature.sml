(* nassau-ugc.23 *)
signature A = sig end and A = sig end
(* CHECK-ERR: × duplicate signature name 'A' *)
(* CHECK-ERR: :2:35] *)
