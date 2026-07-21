structure S : sig type 'a t end = struct type t = int end
(* CHECK-ERR: × type t does not match its specification: the number of type parameters *)
(* CHECK-ERR: :1:35] *)
