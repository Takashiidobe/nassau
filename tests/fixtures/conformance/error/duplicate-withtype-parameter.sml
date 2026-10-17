datatype t = C withtype ('a, 'a) u = 'a
(* CHECK-ERR: × duplicate type variable ''a' *)
(* CHECK-ERR: :1:34] *)
