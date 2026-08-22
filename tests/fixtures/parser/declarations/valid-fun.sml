fun single x = x + 1
fun curried x y = x + y
fun tupled (x, y) = x * y
fun clauses 0 = "zero" | clauses 1 = "one" | clauses _ = "many"
fun fact 0 = 1 | fact n = n * fact (n - 1)
fun even 0 = true | even n = odd (n - 1) and odd 0 = false | odd n = even (n - 1)
fun typed (x : int) : int = x
fun result_type x : int list = [x]
fun cons_pattern (x :: xs) = x | cons_pattern [] = 0
fun layered (all as (a, b)) = a + b
fun several 0 true = 1 | several _ false = 2 | several _ true = 3
fun record_parameter {a, b} = a + b
fun unit_parameter () = 1
fun constant_result x = if x > 0 then "positive" else "other"
(* CHECK-STDOUT: (fun (single ((x) (+ x 1)))) *)
(* CHECK-STDOUT-NEXT: (fun (curried ((x y) (+ x y)))) *)
(* CHECK-STDOUT-NEXT: (fun (tupled (((tuple x y)) {{[(]}}* x y)))) *)
(* CHECK-STDOUT-NEXT: (fun (clauses ((0) "zero") ((1) "one") ((_) "many"))) *)
(* CHECK-STDOUT-NEXT: (fun (fact ((0) 1) ((n) {{[(]}}* n (app fact (- n 1)))))) *)
(* CHECK-STDOUT-NEXT: (fun (even ((0) true) ((n) (app odd (- n 1)))) (odd ((0) false) ((n) (app even (- n 1))))) *)
(* CHECK-STDOUT-NEXT: (fun (typed (((: x int)) (: x int)))) *)
(* CHECK-STDOUT-NEXT: (fun (result_type ((x) (: (list x) (tycon list int))))) *)
(* CHECK-STDOUT-NEXT: (fun (cons_pattern (((:: x xs)) x) (((list)) 0))) *)
(* CHECK-STDOUT-NEXT: (fun (layered (((as all (tuple a b))) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (fun (several ((0 true) 1) ((_ false) 2) ((_ true) 3))) *)
(* CHECK-STDOUT-NEXT: (fun (record_parameter (((record (a a) (b b))) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (fun (unit_parameter ((()) 1))) *)
(* CHECK-STDOUT-NEXT: (fun (constant_result ((x) (if (> x 0) "positive" "other")))) *)
(* CHECK-RUN-EXIT: 0 *)
