val base = 100
functor F (X : sig end) = struct val v = base end
val base = "shadowed"
structure S = F (struct end)
val a = S.v
local
  functor G (X : sig val y : int end) = struct val z = X.y * 2 end
in
  structure T = G (struct val y = 4 end)
end
val b = T.z
(* CHECK-STDOUT: val base : int *)
(* CHECK-STDOUT-NEXT: val base : string *)
(* CHECK-STDOUT-NEXT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int *)
