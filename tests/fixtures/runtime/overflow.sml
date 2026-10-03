(* int arithmetic past Int.maxInt raises Overflow, uncaught here. *)
val _ = print "before\n"
val largest = 1073741823
val next = largest + 1
val _ = print "after\n"
(* ORACLE-INT-PRECISION: 31 *)
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: before *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Overflow with 0 *)
(* CHECK-STDERR-NEXT:  raised at <file overflow.sml> *)
(* CHECK-STDERR-EMPTY: *)
