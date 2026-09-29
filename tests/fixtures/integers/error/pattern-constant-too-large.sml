fun isHuge 1073741824 = true
  | isHuge _ = false
(* CHECK-ERR: × int constant too large *)
(* CHECK-ERR: :1:12] *)
