(* A val binding whose pattern does not match raises Bind. *)
val () = print "before\n"
val SOME x = NONE : int option
val () = print "after\n"
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: before *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Bind with 0 *)
(* CHECK-STDERR-NEXT:  raised at bind-failure.sml:3.5-3.31 *)
(* CHECK-STDERR-EMPTY: *)
