(* mlton regression/slow.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
fun loop (left: IntInf.int): unit =
        case IntInf.compare (left, 0) of
        LESS => ()
        | EQUAL => ()
        | GREATER => loop (left + ~1)

val _ = loop 100000000

val _ = print "All ok\n"
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: All ok *)
