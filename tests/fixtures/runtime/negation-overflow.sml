val smallest = ~1073741824
val product = smallest * ~1
(* ORACLE-INT-PRECISION: 31 *)
(* CHECK-EXIT: 1 *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Overflow with 0 *)
(* CHECK-STDERR-NEXT:  raised at <file negation-overflow.sml> *)
(* CHECK-STDERR-EMPTY: *)
