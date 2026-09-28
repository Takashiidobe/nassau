functor F (X : sig end) = struct end
open F
(* CHECK-ERR: × unbound structure 'F' *)
(* CHECK-ERR: :2:1] *)
