functor F (X : sig val x : int end) = struct end
val y = X.x
(* CHECK-ERR: × unbound structure 'X' *)
(* CHECK-ERR: :2:9] *)
