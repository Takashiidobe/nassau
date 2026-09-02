(* mlton regression/asterisk.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* asterisk.sml *)

(* Checks parsing of "* )". *)

val op* : int * int -> int = (op*);
(* CHECK-EXIT: 0 *)
