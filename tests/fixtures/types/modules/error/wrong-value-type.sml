structure S : sig val f : int -> int end = struct fun f x = x ^ "a" end
(* CHECK-ERR: × value f does not match its specification: the structure has type string -> *)
(* CHECK-ERR: :1:44] *)
