structure S : sig datatype d = A end = struct type d = int val A = 1 end
(* CHECK-ERR: × the structure does not provide datatype d, which the signature specifies *)
(* CHECK-ERR: :1:40] *)
