structure S : sig val id : 'a -> 'a end = struct fun id (x : int) = x end
(* CHECK-ERR: × value id does not match its specification: the structure has type int -> *)
(* CHECK-ERR: :1:43] *)
