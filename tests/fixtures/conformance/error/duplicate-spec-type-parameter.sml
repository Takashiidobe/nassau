signature S = sig type ('a, 'a) t end
(* CHECK-ERR: × duplicate type variable ''a' *)
(* CHECK-ERR: :1:33] *)
