(* mlton regression/exn.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
exception E

fun loop n =
   if n = 0
      then raise E
   else (loop(n - 1) handle e => (print "z"; raise e))

val _ = loop 13 handle _ => print "\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: zzzzzzzzzzzzz *)
