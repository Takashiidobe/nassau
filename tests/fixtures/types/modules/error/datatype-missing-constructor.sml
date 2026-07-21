structure S : sig datatype d = A | B | C end = struct datatype d = A | B end
(* CHECK-ERR: × datatype d does not match its specification: its constructors differ from *)
(* CHECK-ERR: :1:48] *)
