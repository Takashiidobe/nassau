functor F (X : sig end) = struct end
structure A = F (1)
(* CHECK-ERR: × expected a structure expression *)
(* CHECK-ERR: :2:18] *)
