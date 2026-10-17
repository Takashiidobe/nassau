val 'a f = fn (x : 'a) => x and g = fn (y : 'a) => y + 1
(* CHECK-ERR: × expected 'a, found 'b *)
(* CHECK-ERR: :1:52] *)
