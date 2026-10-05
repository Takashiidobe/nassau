(* mlton regression/3.sml @5fe943391; HPND licence in tests/fixtures/mlton/LICENSE *)
(* Lexing tyvars *)

type ('_a, '1a, ''a, '_, '', ''', ''1) t = int

type u = (int, int, int, int, int, int, int) t
(* CHECK-EXIT: 0 *)
