(* nassau-ugc.25 *)
val x = let val id:'a -> 'a = fn z => z in id id end
val _ = print (x "scope\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: scope *)
