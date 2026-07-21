structure S : sig eqtype t end = struct type t = int -> int end
(* CHECK-ERR: × type t does not match its specification: it is specified with eqtype but *)
(* CHECK-ERR: :1:34] *)
