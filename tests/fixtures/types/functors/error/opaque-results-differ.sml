functor F (X : sig end) :> sig type t val x : t end = struct type t = int val x = 1 end
structure A = F (struct end)
structure B = F (struct end)
val y = [A.x, B.x]
(* SMLNJ-SKIP: SML/NJ 110.99.9 incorrectly accepts opaque functor results *)
(* CHECK-ERR: × expected A.t, found B.t *)
(* CHECK-ERR: :4:15] *)
