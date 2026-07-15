fun same (a, b) = a = b
fun differ (a, b) = a <> b
fun mem x [] = false | mem x (y :: ys) = x = y orelse mem x ys
val strings = "a" = "b"
val lists = [1, 2] = [1, 2]
val tuples = (1, "a") = (1, "a")
val options = SOME 1 = NONE
val cells = ref 1 = ref 1
fun both_eq (x, y) = (x = x, y)
fun nested x = [x] = [x]
val units = () = ()
(* CHECK-STDOUT: val same : ''a * ''a -> bool *)
(* CHECK-STDOUT-NEXT: val differ : ''a * ''a -> bool *)
(* CHECK-STDOUT-NEXT: val mem : ''a -> ''a list -> bool *)
(* CHECK-STDOUT-NEXT: val strings : bool *)
(* CHECK-STDOUT-NEXT: val lists : bool *)
(* CHECK-STDOUT-NEXT: val tuples : bool *)
(* CHECK-STDOUT-NEXT: val options : bool *)
(* CHECK-STDOUT-NEXT: val cells : bool *)
(* CHECK-STDOUT-NEXT: val both_eq : ''a * 'b -> bool * 'b *)
(* CHECK-STDOUT-NEXT: val nested : ''a -> bool *)
(* CHECK-STDOUT-NEXT: val units : bool *)
