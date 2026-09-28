functor F (X : sig end) : sig val x : int end = struct val x = "s" end
(* CHECK-ERR: × value x does not match its specification: the structure has type string *)
(* CHECK-ERR: :1:49] *)
