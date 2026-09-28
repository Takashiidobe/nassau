val bad = if true then [1] else [true]
(* CHECK-ERR: × expected int list, found bool list *)
(* CHECK-ERR: :1:33] *)
