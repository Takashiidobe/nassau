structure S : sig val f : int -> int end = struct val f = 3 end
(* CHECK-ERR: × value f does not match its specification: the structure has type int but *)
(* CHECK-ERR: :1:44] *)
