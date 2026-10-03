val base = 100
functor F (X : sig end) = struct val v = base end
val base = "shadowed"
structure S = F (struct end)
val a = S.v
functor G (X : sig val y : int end) = struct val z = X.y * 2 end
local
  val y = 4
in
  structure T = G (struct val y = y end)
end
val b = T.z
val () = print (if a = 100 andalso b = 8 then "valid-functor-scope\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-functor-scope *)
