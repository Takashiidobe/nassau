(* mlton regression/time2.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
open Time

val _ =
   if 1000000000000 = toMilliseconds (fromSeconds 1000000000)
      then print "OK\n"
   else print "ERROR\n"

val _ =
   if 1000000000000 = toMicroseconds (fromSeconds 1000000)
      then print "OK\n"
   else print "ERROR\n"

val _ =
   if 1000000000 = toSeconds (fromMilliseconds 1000000000000)
      then print "OK\n"
   else print "ERROR\n"

val _ =
   if 1000000000 = toSeconds (fromMicroseconds 1000000000000000)
      then print "OK\n"
   else print "ERROR\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: OK *)
(* CHECK-STDOUT-NEXT: OK *)
(* CHECK-STDOUT-NEXT: OK *)
(* CHECK-STDOUT-NEXT: OK *)
