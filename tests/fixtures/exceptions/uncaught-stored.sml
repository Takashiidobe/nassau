(* A value raised again later, after being caught and stored, still names
   where it was first raised; an argument that is a block is <unknown>. *)
exception Pair of int * int
val saved = ref (Pair (0, 0))
val () = (raise Pair (1, 2)) handle e => saved := e
val () = print "saved\n"
val () = raise (!saved)
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: saved *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Pair with <unknown> raised at uncaught-stored.sml:5.17-5.28 *)
(* CHECK-STDERR-EMPTY: *)
