fun literal 0 = 1
fun option (SOME x) = x
fun boolean true = 1
fun pairs 0 true = 1 | pairs 1 false = 2
fun total 0 = 1 | total n = n
(* CHECK-STDOUT: (fun (literal ((0) 1))) *)
(* CHECK-STDOUT-NEXT: (fun (option (((con SOME x)) x))) *)
(* CHECK-STDOUT-NEXT: (fun (boolean ((true) 1))) *)
(* CHECK-STDOUT-NEXT: (fun (pairs ((0 true) 1) ((1 false) 2))) *)
(* CHECK-STDOUT-NEXT: (fun (total ((0) 1) ((n) n))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 1:13 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 2:13 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 3:13 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 4:11 *)
