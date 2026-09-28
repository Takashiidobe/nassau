functor F (X : sig end) = struct end
structure A = F (struct end
(* CHECK-ERR: × expected ) after the functor argument *)
(* CHECK-ERR: :2:25] *)
