val cell = ref 0;
val kept = 3; val failed = (cell := 7; raise Fail "stop");
val after = kept + !cell;
datatype t = A | B of int;
val fresh = B after;
(* ORACLE-EXIT: 1 *)
(* CHECK-REPL: val cell = ref 0 : int ref *)
(* CHECK-REPL: val kept = 3 : int *)
(* CHECK-REPL-EMPTY: *)
(* CHECK-REPL: uncaught exception Fail {{[[]}}Fail: stop] *)
(* CHECK-REPL:   raised at: stdIn:2.46-2.57 *)
(* CHECK-REPL: val after = 10 : int *)
(* CHECK-REPL: datatype t = A | B of int *)
(* CHECK-REPL: val fresh = B 10 : t *)
