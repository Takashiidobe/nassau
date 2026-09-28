val x = 1 handle Div => "zero"
(* CHECK-ERR: × expected int, found string *)
(* CHECK-ERR: :1:25] *)
