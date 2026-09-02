(* mlton regression/14.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
datatype t = A | B
datatype u = C of t

val _ = if C A = C B then raise Fail "bug" else ()
(* CHECK-EXIT: 0 *)
