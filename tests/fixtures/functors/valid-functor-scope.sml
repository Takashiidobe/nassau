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
val () = print (if a = 100 andalso b = 8 then "valid-functor-scope\n" else "wrong\n")
(* MLTON-SKIP: MLton rejects functors declared inside local *)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-functor-scope *)
