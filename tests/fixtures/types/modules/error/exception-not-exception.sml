structure S : sig exception E end = struct val E = 1 end
(* CHECK-ERR: × the structure does not provide exception E, which the signature specifies *)
(* CHECK-ERR: :1:37] *)
