fun head xs = let val h :: _ = xs in h end
val () = print (Int.toString (head [1, 2]) ^ "\n")
val () = print (Int.toString (head []) ^ "\n")
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: 1 *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Bind with 0 *)
(* CHECK-STDERR-NEXT:  raised at let-bind-failure.sml:1.24-1.35 *)
(* CHECK-STDERR-EMPTY: *)
