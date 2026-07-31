fun f (SOME [x], (y, true)) = x + y
  | f (NONE, (y, false)) = y
val () = print (Int.toString (f (SOME [1], (2, true))) ^ "\n")
val () = print (Int.toString (f (NONE, (3, false))) ^ "\n")
val () = print (Int.toString (f (SOME [1, 2], (2, true))) ^ "\n")
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Match with 0 *)
(* CHECK-STDERR-NEXT:  raised at nested-failure.sml:2.29 *)
(* CHECK-STDERR-EMPTY: *)
