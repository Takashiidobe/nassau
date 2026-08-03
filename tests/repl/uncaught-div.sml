(* The basis's exceptions describe themselves when uncaught. *)
val a = 7;
val b = a div 0;
(* CHECK-REPL: val a = 7 : int *)
(* CHECK-REPL-EMPTY: *)
(* CHECK-REPL-NEXT: uncaught exception Div [divide by zero] *)
(* CHECK-REPL-NEXT:   raised at: stdIn:2.11-2.14 *)
