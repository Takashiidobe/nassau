datatype t = A
val x : int t = A
(* CHECK-ERR: × type 't' expects 0 type argument(s), found 1 *)
(* CHECK-ERR: :2:9] *)
