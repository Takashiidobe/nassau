val partial = (raise Fail "x") handle Fail "x" => 0
val named = (1 div 0) handle Div => 0 | Overflow => 1
(* CHECK-STDOUT: (val partial (handle (raise (app Fail "x")) ((con Fail "x") 0))) *)
(* CHECK-STDOUT-NEXT: (val named (handle (div 1 0) (Div 0) (Overflow 1))) *)
(* CHECK-RUN-EXIT: 0 *)
