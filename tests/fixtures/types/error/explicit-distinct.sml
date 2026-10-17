fun ('a, 'b) bad (x : 'a) : 'b = x
(* CHECK-ERR: × expected 'a, found 'b *)
(* CHECK-ERR: :1:34] *)
