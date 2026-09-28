structure S : sig datatype d = A of int end = struct datatype d = A of string end
(* CHECK-ERR: × constructor A does not match its specification: the structure has type *)
(* CHECK-ERR: :1:47] *)
