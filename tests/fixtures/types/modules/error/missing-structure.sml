structure S : sig structure Inner : sig val x : int end end = struct val x = 1 end
(* CHECK-ERR: × the structure does not provide structure Inner, which the signature *)
(* CHECK-ERR: :1:63] *)
