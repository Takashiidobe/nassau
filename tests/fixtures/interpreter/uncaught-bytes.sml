val () = raise Fail "a\255b"
(* CHECK-EXIT: 1 *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Fail with "a{{.}}b" raised at uncaught-bytes.sml:{{[0-9.:-]+}} *)
(* CHECK-STDERR-EMPTY: *)
