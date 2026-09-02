(* mlton regression/int-overflow.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
val _ =
   (Int.fromString "12345678901234567890"
    ; print "ERROR\n")
   handle Overflow => print "OK\n"

val min = valOf Int.minInt
val _ =
   (~ min
    ; print "ERROR\n")
   handle Overflow => print "OK\n"

val _ =
   (Int.fromString "2147483648"
    ; print "ERROR\n")
   handle Overflow => print "OK\n"

val _ =
   (abs min
    ; print "ERROR\n")
   handle Overflow => print "OK\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: OK *)
(* CHECK-STDOUT-NEXT: OK *)
(* CHECK-STDOUT-NEXT: ERROR *)
(* CHECK-STDOUT-NEXT: OK *)
