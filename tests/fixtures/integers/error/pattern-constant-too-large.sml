fun isHuge 1073741824 = true
  | isHuge _ = false
(* SMLNJ-INT-PRECISION: 31 *)
(* CHECK-ERR: × int constant too large *)
(* CHECK-ERR: :1:12] *)
