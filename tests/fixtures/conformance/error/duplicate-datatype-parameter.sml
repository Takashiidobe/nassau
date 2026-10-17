datatype ('a, 'a) t = C of 'a
(* CHECK-ERR: × duplicate type variable ''a' *)
(* CHECK-ERR: :1:19] *)
