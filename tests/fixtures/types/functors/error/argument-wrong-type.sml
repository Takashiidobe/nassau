functor F (X : sig val x : int end) = struct val y = X.x end
structure A = F (struct val x = "s" end)
(* CHECK-ERR: × value x does not match its specification: the structure has type string *)
(* CHECK-ERR: :2:18] *)
