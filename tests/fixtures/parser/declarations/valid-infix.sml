infix 6 ++
fun x ++ y = x + y
val three = 1 ++ 2
val mixed = 1 ++ 2 * 3
val left = 10 ++ 4 ++ 3
infixr 5 ##
fun a ## b = a - b
val right = 10 ## 4 ## 3
infix 7 **
fun (a ** b) c = a * b * c
val product = (2 ** 3) 4
fun op --- (a, b) = a - b
val as_prefix = op --- (5, 2)
infix 3 <<>>
fun a <<>> b = a
val default_precedence = 1 <<>> 2
nonfix ++
val prefix = ++ (1, 2)
(* CHECK-STDOUT: (infix 6 ++) *)
(* CHECK-STDOUT-NEXT: (fun (++ (((tuple x y)) (+ x y)))) *)
(* CHECK-STDOUT-NEXT: (val three (++ 1 2)) *)
(* CHECK-STDOUT-NEXT: (val mixed (++ 1 {{[(]}}* 2 3))) *)
(* CHECK-STDOUT-NEXT: (val left (++ (++ 10 4) 3)) *)
(* CHECK-STDOUT-NEXT: (infixr 5 ##) *)
(* CHECK-STDOUT-NEXT: (fun (## (((tuple a b)) (- a b)))) *)
(* CHECK-STDOUT-NEXT: (val right (## 10 (## 4 3))) *)
(* CHECK-STDOUT-NEXT: (infix 7 **{{[)]}} *)
(* CHECK-STDOUT-NEXT: (fun {{[(]}}** (((tuple a b) c) {{[(]}}* {{[(]}}* a b) c)))) *)
(* CHECK-STDOUT-NEXT: (val product (app {{[(]}}** 2 3) 4)) *)
(* CHECK-STDOUT-NEXT: (fun (--- (((tuple a b)) (- a b)))) *)
(* CHECK-STDOUT-NEXT: (val as_prefix (app --- (tuple 5 2))) *)
(* CHECK-STDOUT-NEXT: (infix 3 <<>>) *)
(* CHECK-STDOUT-NEXT: (fun (<<>> (((tuple a b)) a))) *)
(* CHECK-STDOUT-NEXT: (val default_precedence (<<>> 1 2)) *)
(* CHECK-STDOUT-NEXT: (nonfix ++) *)
(* CHECK-STDOUT-NEXT: (val prefix (app ++ (tuple 1 2))) *)
