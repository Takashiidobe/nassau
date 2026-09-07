val xs = [1, 2, 3]
val precedence = 1 + 2 * 3 - 4
val left_minus = 20 - 5 - 3
val left_div = 100 div 5 div 2
val cons = 1 :: 2 :: []
val append = ["a"] @ ["b"] @ ["c"]
val mixed = ["a"] @ ["b" ^ "c"]
val comparison = 1 + 2 < 4
val chained = (1 < 2) = true
val negative = 3 - ~2
val prefix_op = op + (1, 2)
val modulo = 10 mod 3 * 2
val composed = (op ~ o op ~) 5
(* RUNTIME-SKIP: unbound variable '~' *)
(* CHECK-STDOUT: (val xs (list 1 2 3)) *)
(* CHECK-STDOUT-NEXT: (val precedence (- (+ 1 {{[(]}}* 2 3)) 4)) *)
(* CHECK-STDOUT-NEXT: (val left_minus (- (- 20 5) 3)) *)
(* CHECK-STDOUT-NEXT: (val left_div (div (div 100 5) 2)) *)
(* CHECK-STDOUT-NEXT: (val cons (:: 1 (:: 2 (list)))) *)
(* CHECK-STDOUT-NEXT: (val append (@ (list "a") (@ (list "b") (list "c")))) *)
(* CHECK-STDOUT-NEXT: (val mixed (@ (list "a") (list (^ "b" "c")))) *)
(* CHECK-STDOUT-NEXT: (val comparison (< (+ 1 2) 4)) *)
(* CHECK-STDOUT-NEXT: (val chained (= (< 1 2) true)) *)
(* CHECK-STDOUT-NEXT: (val negative (- 3 -2)) *)
(* CHECK-STDOUT-NEXT: (val prefix_op (app (fn ((tuple x y) (+ x y))) (tuple 1 2))) *)
(* CHECK-STDOUT-NEXT: (val modulo {{[(]}}* (mod 10 3) 2)) *)
(* CHECK-STDOUT-NEXT: (val composed (app (o ~ ~) 5)) *)
(* CHECK-RUN-ERR: × unbound variable '~' *)
(* CHECK-RUN-ERR: :13:17] *)
