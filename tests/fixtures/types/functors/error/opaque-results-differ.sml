functor F (X : sig end) :> sig type t val x : t end = struct type t = int val x = 1 end
structure A = F (struct end)
structure B = F (struct end)
val y = [A.x, B.x]
(* CHECK-ERR: × expected A.t, found B.t *)
(* CHECK-ERR: :4:15] *)
