type t = int
type 'a pair = 'a * 'a
type ('a, 'b) either = 'a * 'b
type a = int and b = string
type fn_type = int -> int
type record = {x : int, y : int}
val x : t = 1
val p : int pair = (1, 2)
val r : record = {x = 1, y = 2}
(* CHECK-STDOUT: (type (t () int)) *)
(* CHECK-STDOUT-NEXT: (type (pair ('a) {{[(]}}* 'a 'a))) *)
(* CHECK-STDOUT-NEXT: (type (either ('a 'b) {{[(]}}* 'a 'b))) *)
(* CHECK-STDOUT-NEXT: (type (a () int) (b () string)) *)
(* CHECK-STDOUT-NEXT: (type (fn_type () (-> int int))) *)
(* CHECK-STDOUT-NEXT: (type (record () (record (x int) (y int)))) *)
(* CHECK-STDOUT-NEXT: (val (: x t) 1) *)
(* CHECK-STDOUT-NEXT: (val (: p (tycon pair int)) (tuple 1 2)) *)
(* CHECK-STDOUT-NEXT: (val (: r record) (record (x 1) (y 2))) *)
(* CHECK-RUN-EXIT: 0 *)
