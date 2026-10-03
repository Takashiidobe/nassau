val smallest = ~1073741824
val tooSmall = ~1073741825
(* ORACLE-INT-PRECISION: 31 *)
(* CHECK-ERR: × int constant too large *)
(* CHECK-ERR: :2:16] *)
