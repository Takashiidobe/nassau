functor F (X : sig type t val x : t end) = struct val y = X.x + 1 end
(* CHECK-ERR: × operator is not defined for type X.t *)
(* CHECK-ERR: :1:59] *)
