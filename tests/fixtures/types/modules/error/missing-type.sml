structure S : sig type t val x : int end = struct val x = 1 end
(* CHECK-ERR: × the structure does not provide type t, which the signature specifies *)
(* CHECK-ERR: :1:44] *)
