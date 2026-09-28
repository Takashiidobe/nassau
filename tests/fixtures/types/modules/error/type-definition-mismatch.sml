structure S : sig type t = int end = struct type t = string end
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :1:38] *)
