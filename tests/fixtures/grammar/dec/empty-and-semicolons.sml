(* SML'97 grammar, declaration: empty and semicolons. *)
fun pi n = print (Int.toString n ^ "\n")
;; val x = 1 ; val y = 2 ;
val _ = pi (x + y);
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
