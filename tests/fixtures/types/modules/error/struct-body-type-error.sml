structure S = struct val x = 1 + "a" end
(* CHECK-ERR: × expected int, found string *)
(* CHECK-ERR: :1:34] *)
