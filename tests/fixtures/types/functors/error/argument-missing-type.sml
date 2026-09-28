functor F (X : sig type t end) = struct end
structure A = F (struct end)
(* CHECK-ERR: × the structure does not provide type t, which the signature specifies *)
(* CHECK-ERR: :2:18] *)
