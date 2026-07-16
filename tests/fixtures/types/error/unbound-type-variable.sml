datatype 'a t = A of 'b
(* CHECK-ERR: × unbound type variable 'b in type declaration *)
(* CHECK-ERR: :1:22] *)
