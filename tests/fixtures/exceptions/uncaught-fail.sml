(* A string argument is shown on the same line as the position. *)
fun fail what = raise Fail ("no " ^ what)
val _ = fail "way"
(* CHECK-EXIT: 1 *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Fail with "no way" raised at uncaught-fail.sml:2.23-2.42 *)
(* CHECK-STDERR-EMPTY: *)
