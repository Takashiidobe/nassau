(* nassau-949.24 *)
val _ = print (if 0w1 + 0w2 = 0w3 then "word\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: word *)
