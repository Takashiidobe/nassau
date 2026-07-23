functor F (val x : int) = struct val y = x end
structure A = F (val x = true)
(* CHECK-ERR: × value x does not match its specification: the structure has type bool but *)
(* CHECK-ERR: :2:18] *)
