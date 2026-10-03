functor F (X : sig end) :> sig type t val x : t end = struct type t = int val x = 1 end
structure A = F (struct end)
val y = A.x + 1
(* ORACLE: mlton *)
(* CHECK-ERR: × operator is not defined for type A.t *)
(* CHECK-ERR: :3:9] *)
