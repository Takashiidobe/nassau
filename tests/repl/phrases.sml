val x = 1 val x = 2;
val a = 1; val b = a + 2;
val tuples = (a; b);
val scoped = let val xs = [a]; val ys = [b] in (xs, ys) end;
val () = print "semicolon;inside\n";
(* ORACLE-REPL *)
(* CHECK-REPL: val x = 2 : int *)
(* CHECK-REPL: val a = 1 : int *)
(* CHECK-REPL: val b = 3 : int *)
(* CHECK-REPL: val tuples = 3 : int *)
(* CHECK-REPL: val scoped = ({{[[]}}1],{{[[]}}3]) : int list * int list *)
(* CHECK-REPL: semicolon;inside *)
