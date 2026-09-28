local functor F (X : sig end) = struct end in end
structure A = F (struct end)
(* CHECK-ERR: × unbound functor 'F' *)
(* CHECK-ERR: :2:15] *)
