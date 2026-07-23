val f = let datatype t = A in A end
val x : int = f
(* CHECK-ERR: × expected int, found ?.t *)
(* CHECK-ERR: :2:15] *)
