(* int is 31 bits wide, as in SML/NJ: Int.maxInt is 1073741823. *)
val largest = 1073741823
val tooLarge = 1073741824
(* ORACLE-INT-PRECISION: 31 *)
(* CHECK-ERR: × int constant too large *)
(* CHECK-ERR: :3:16] *)
