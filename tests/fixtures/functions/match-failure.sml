(* A non-exhaustive function raises Match. *)
fun positive 1 = "one"
  | positive 2 = "two"
val _ = print (positive 2 ^ "\n")
val _ = print (positive 3 ^ "\n")
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: two *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Match with 0 *)
(* CHECK-STDERR-NEXT:  raised at match-failure.sml:3.23 *)
(* CHECK-STDERR-EMPTY: *)
