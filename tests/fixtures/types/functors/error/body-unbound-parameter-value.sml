functor F (X : sig val x : int end) = struct val y = X.z end
(* CHECK-ERR: × unbound variable 'X.z' *)
(* CHECK-ERR: :1:54] *)
