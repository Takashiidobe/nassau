structure S : sig eqtype t end = struct datatype t = F of int -> int end
(* CHECK-ERR: × type t does not match its specification: it is specified with eqtype but *)
(* CHECK-ERR: :1:34] *)
