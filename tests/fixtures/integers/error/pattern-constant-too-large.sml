fun isHuge 1073741824 = true
  | isHuge _ = false
(* ORACLE-INT-PRECISION: 31 *)
(* CHECK-ERR: × int constant too large *)
(* CHECK-ERR: :1:12] *)
