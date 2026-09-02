(* mlton regression/semicolon.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* semicolon.sml *)

(* Checks parsing of semicolons. *)

structure A = struct ;;;;;;;; end;
signature S = sig ;;;;;;;;;;; end;

;;;;;;;;;;;;;;;;
(* CHECK-EXIT: 0 *)
