val it = 10;
it + 1;
val saved = it;
(print (Int.toString saved ^ "\n"); saved + 2);
val _ = print (Int.toString it ^ "\n");
let val x = 4 in x + 1 end;
val _ = print (Int.toString it ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 11 *)
(* CHECK-STDOUT-NEXT: 13 *)
(* CHECK-STDOUT-NEXT: 5 *)
