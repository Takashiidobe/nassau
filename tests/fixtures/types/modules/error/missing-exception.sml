structure S : sig exception Oops end = struct end
(* CHECK-ERR: × the structure does not provide exception Oops, which the signature *)
(* CHECK-ERR: :1:40] *)
