fun 'a bad (x : 'a) = let val y : 'a = 1 in x end
(* CHECK-ERR: × expected 'a, found int *)
(* CHECK-ERR: :1:40] *)
