val unwrap = fn SOME x => x
val () = print (Int.toString (unwrap (SOME 5)) ^ "\n")
val () = print (Int.toString (unwrap NONE) ^ "\n")
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: 5 *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Match with 0 *)
(* CHECK-STDERR-NEXT:  raised at fn-failure.sml:1.29 *)
(* CHECK-STDERR-EMPTY: *)
