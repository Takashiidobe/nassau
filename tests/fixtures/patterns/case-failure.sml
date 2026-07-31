fun name n =
  case n of
      1 => "one"
    | 2 => "two"
val () = print (name 1 ^ "\n")
val () = print (name 3 ^ "\n")
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: one *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Match with 0 *)
(* CHECK-STDERR-NEXT:  raised at case-failure.sml:4.17 *)
(* CHECK-STDERR-EMPTY: *)
