functor F (X : sig end) = struct val y = 1 + "a" end
(* CHECK-ERR: × expected int, found string *)
(* CHECK-ERR: :1:46] *)
