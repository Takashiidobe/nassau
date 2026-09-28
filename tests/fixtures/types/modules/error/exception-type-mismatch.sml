structure S : sig exception E of int end = struct exception E of string end
(* CHECK-ERR: × exception E does not match its specification: the structure has type *)
(* CHECK-ERR: :1:44] *)
