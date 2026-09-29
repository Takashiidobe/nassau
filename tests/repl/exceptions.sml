(* Exceptions in the REPL: handled ones as in a program; one that escapes *)
(* a declaration is reported with SML/NJ's message, and SML/NJ then *)
(* discards the rest of its input, so it comes last. *)
fun check n = if n > 3 then raise Fail "too big" else n;
val handled = check 7 handle Fail m => size m;
val zero = 1 div 0 handle Div => 0;
val _ = (print "before\n"; check 9);
(* CHECK-REPL: val check = fn : int -> int *)
(* CHECK-REPL-NEXT: val handled = 7 : int *)
(* CHECK-REPL-NEXT: val zero = 0 : int *)
(* CHECK-REPL-NEXT: before *)
(* CHECK-REPL-EMPTY: *)
(* CHECK-REPL-NEXT: uncaught exception Fail [Fail: too big] *)
(* CHECK-REPL-NEXT:   raised at: stdIn:1.36-1.50 *)
