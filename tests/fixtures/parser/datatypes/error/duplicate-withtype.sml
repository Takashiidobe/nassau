datatype t = A withtype t = int
(* CHECK-ERR: × duplicate type name 't' in type declaration *)
(* CHECK-ERR: :1:1] *)
