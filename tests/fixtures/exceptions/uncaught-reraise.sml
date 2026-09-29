(* An exception keeps the position it was first raised at, whether a
   handler's rules pass it on or a handler raises it again. *)
exception Oops
exception Other
fun inner () = raise Oops
fun middle () = inner () handle Other => ()
fun outer () = middle () handle e => (print "cleaning up\n"; raise e)
val () = outer ()
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: cleaning up *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Oops with 0 *)
(* CHECK-STDERR-NEXT:  raised at uncaught-reraise.sml:5.22-5.26 *)
(* CHECK-STDERR-EMPTY: *)
