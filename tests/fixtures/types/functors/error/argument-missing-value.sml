functor F (X : sig val x : int end) = struct val y = X.x end
structure A = F (struct val z = 1 end)
(* CHECK-ERR: × the structure does not provide value x, which the signature specifies *)
(* CHECK-ERR: :2:18] *)
