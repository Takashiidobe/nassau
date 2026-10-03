datatype color = Red | Green | Blue
datatype 'a tree = Leaf | Node of 'a tree * 'a * 'a tree
datatype ('a, 'b) either = Left of 'a | Right of 'b
datatype shape = Circle of real | Rect of {width : real, height : real} | Point
datatype fn_box = Box of int -> int
datatype 'a nested = Nil | Cons of 'a * 'a nested list
datatype t = op A | op B of int
(* CHECK-STDOUT: (datatype (color () (Red) (Green) (Blue))) *)
(* CHECK-STDOUT-NEXT: (datatype (tree ('a) (Leaf) (Node {{[(]}}* (tycon tree 'a) 'a (tycon tree 'a))))) *)
(* CHECK-STDOUT-NEXT: (datatype (either ('a 'b) (Left 'a) (Right 'b))) *)
(* CHECK-STDOUT-NEXT: (datatype (shape () (Circle real) (Rect (record (width real) (height real))) (Point))) *)
(* CHECK-STDOUT-NEXT: (datatype (fn_box () (Box (-> int int)))) *)
(* CHECK-STDOUT-NEXT: (datatype (nested ('a) (Nil) (Cons {{[(]}}* 'a (tycon list (tycon nested 'a)))))) *)
(* CHECK-STDOUT-NEXT: (datatype (t () (A) (B int))) *)
(* CHECK-RUN-EXIT: 0 *)
