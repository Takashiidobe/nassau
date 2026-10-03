val xs = [1, 2, 3]
val conditional = if 1 < 2 then 10 else 20
val short_circuit = if 1 < 2 andalso 2 < 3 orelse false then 1 else 2
val nested = if true then if false then 1 else 2 else 3
val sequence = (print "a"; print "b"; 3)
val loop = while false do ()
val guarded = (raise Fail "x") handle _ => 1
val many_handlers = (raise Fail "x") handle Div => 1 | _ => 2
(* CHECK-STDOUT: (val xs (list 1 2 3)) *)
(* CHECK-STDOUT-NEXT: (val conditional (if (< 1 2) 10 20)) *)
(* CHECK-STDOUT-NEXT: (val short_circuit (if (orelse (andalso (< 1 2) (< 2 3)) false) 1 2)) *)
(* CHECK-STDOUT-NEXT: (val nested (if true (if false 1 2) 3)) *)
(* CHECK-STDOUT-NEXT: (val sequence (seq (app print "a") (app print "b") 3)) *)
(* CHECK-STDOUT-NEXT: (val loop (while false ())) *)
(* CHECK-STDOUT-NEXT: (val guarded (handle (raise (app Fail "x")) (_ 1))) *)
(* CHECK-STDOUT-NEXT: (val many_handlers (handle (raise (app Fail "x")) (Div 1) (_ 2))) *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: ab *)
