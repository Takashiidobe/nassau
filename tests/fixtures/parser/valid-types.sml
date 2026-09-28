val xs = [1, 2, 3]
val annotated = 1 : int
val listed = [] : int list
val nested_list = [[1]] : int list list
val function = (fn x => x) : int -> int
val higher = (fn f => f 1) : (int -> int) -> int
val product = (1, "a") : int * string
val record = {a = 1} : {a : int}
val variable = [] : 'a list
val chained = 1 : int : int
(* CHECK-STDOUT: (val xs (list 1 2 3)) *)
(* CHECK-STDOUT-NEXT: (val annotated (: 1 int)) *)
(* CHECK-STDOUT-NEXT: (val listed (: (list) (tycon list int))) *)
(* CHECK-STDOUT-NEXT: (val nested_list (: (list (list 1)) (tycon list (tycon list int)))) *)
(* CHECK-STDOUT-NEXT: (val function (: (fn (x x)) (-> int int))) *)
(* CHECK-STDOUT-NEXT: (val higher (: (fn (f (app f 1))) (-> (-> int int) int))) *)
(* CHECK-STDOUT-NEXT: (val product (: (tuple 1 "a") {{[(]}}* int string))) *)
(* CHECK-STDOUT-NEXT: (val record (: (record (a 1)) (record (a int)))) *)
(* CHECK-STDOUT-NEXT: (val variable (: (list) (tycon list 'a))) *)
(* CHECK-STDOUT-NEXT: (val chained (: (: 1 int) int)) *)
