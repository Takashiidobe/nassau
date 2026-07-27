(* div by zero raises Div, reported at the operator. *)
val _ = print "dividing\n"
val zero = 0
val quotient = 10 div zero
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: dividing *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Div with 0 *)
(* CHECK-STDERR-NEXT:  raised at div-by-zero.sml:4.19-4.22 *)
(* CHECK-STDERR-EMPTY: *)
