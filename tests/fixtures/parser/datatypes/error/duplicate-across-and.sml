datatype t = A | B
and u = C | A
(* CHECK-ERR: × duplicate constructor name 'A' in datatype declaration *)
(* CHECK-ERR: :2:13] *)
