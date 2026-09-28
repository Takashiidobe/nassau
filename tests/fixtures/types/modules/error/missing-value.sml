structure S : sig val f : int -> int val g : int end = struct fun f x = x end
(* CHECK-ERR: × the structure does not provide value g, which the signature specifies *)
(* CHECK-ERR: :1:56] *)
