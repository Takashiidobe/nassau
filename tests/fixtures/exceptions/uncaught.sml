(* An exception that escapes the program is reported on stderr as SML/NJ
   reports it, with its argument when that is an immediate, after the output
   so far. *)
exception Code of int
fun check n = if n > 3 then raise Code (0 - n) else n
val () = print (Int.toString (check 2) ^ "\n")
val _ = check 5
val () = print "not reached\n"
(* CHECK-EXIT: 1 *)
(* CHECK-STDOUT: 2 *)
(* CHECK-STDERR: /usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception Code with -5 *)
(* CHECK-STDERR-NEXT:  raised at uncaught.sml:5.35-5.47 *)
(* CHECK-STDERR-EMPTY: *)
