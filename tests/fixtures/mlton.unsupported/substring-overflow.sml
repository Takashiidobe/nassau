(* mlton regression/substring-overflow.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
open Substring

val _ =
   (slice (full "abc", 1, SOME (valOf Int.maxInt))
    ; print "ERROR\n")
   handle Subscript => print "OK\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: OK *)
