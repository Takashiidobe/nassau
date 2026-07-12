val bad = if true then [1] else [true]
(* CHECK-ERR: × expected int, found bool *)
(* CHECK-ERR: :1:11] *)
