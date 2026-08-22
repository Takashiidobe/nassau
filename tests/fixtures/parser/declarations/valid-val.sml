val pair = (1, 2)
val (a, b) = pair
val (c, d) = (3, (4, 5))
val {g, h = i} = {g = 1, h = 2}
val j = 1 and k = 2
val (l, m) = (1, 2) and n = 3
val whole as (u, v) = (1, 2)
val _ = 1
val typed : int = 1
val rec fact = fn 0 => 1 | n => n * fact (n - 1)
val rec even = fn 0 => true | n => odd (n - 1) and odd = fn 0 => false | n => even (n - 1)
val shadow = 1
val shadow = shadow + 1
val [q, r] = [1, 2]
val s :: rest = [1, 2, 3]
val SOME t = SOME 4
(* CHECK-STDOUT: (val pair (tuple 1 2)) *)
(* CHECK-STDOUT-NEXT: (val (tuple a b) pair) *)
(* CHECK-STDOUT-NEXT: (val (tuple c d) (tuple 3 (tuple 4 5))) *)
(* CHECK-STDOUT-NEXT: (val (record (g g) (h i)) (record (g 1) (h 2))) *)
(* CHECK-STDOUT-NEXT: (val-and (j 1) (k 2)) *)
(* CHECK-STDOUT-NEXT: (val-and ((tuple l m) (tuple 1 2)) (n 3)) *)
(* CHECK-STDOUT-NEXT: (val (as whole (tuple u v)) (tuple 1 2)) *)
(* CHECK-STDOUT-NEXT: (val _ 1) *)
(* CHECK-STDOUT-NEXT: (val (: typed int) 1) *)
(* CHECK-STDOUT-NEXT: (val-rec (fact (fn (0 1) (n {{[(]}}* n (app fact (- n 1))))))) *)
(* CHECK-STDOUT-NEXT: (val-rec (even (fn (0 true) (n (app odd (- n 1))))) (odd (fn (0 false) (n (app even (- n 1)))))) *)
(* CHECK-STDOUT-NEXT: (val shadow 1) *)
(* CHECK-STDOUT-NEXT: (val shadow (+ shadow 1)) *)
(* CHECK-STDOUT-NEXT: (val (list q r) (list 1 2)) *)
(* CHECK-STDOUT-NEXT: (val (:: s rest) (list 1 2 3)) *)
(* CHECK-STDOUT-NEXT: (val (con SOME t) (app SOME 4)) *)
(* CHECK-RUN-EXIT: 0 *)
