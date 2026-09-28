datatype color = Red | Green | Blue
fun full Red = 1 | full Green = 2 | full Blue = 3
fun missing Red = 1 | missing Green = 2
fun guarded Red = 1 | guarded _ = 2
datatype 'a option2 = None | Some of 'a
fun both (None, None) = 0 | both (Some _, None) = 1 | both (None, Some _) = 2 | both (Some _, Some _) = 3
fun half (Some x) = x
val by_case = fn c => case c of Red => "r" | Green => "g"
fun nested (Some (Some x)) = x | nested (Some None) = 0 | nested None = 0
(* CHECK-STDOUT: (datatype (color () (Red) (Green) (Blue))) *)
(* CHECK-STDOUT-NEXT: (fun (full ((Red) 1) ((Green) 2) ((Blue) 3))) *)
(* CHECK-STDOUT-NEXT: (fun (missing ((Red) 1) ((Green) 2))) *)
(* CHECK-STDOUT-NEXT: (fun (guarded ((Red) 1) ((_) 2))) *)
(* CHECK-STDOUT-NEXT: (datatype (option2 ('a) (None) (Some 'a))) *)
(* CHECK-STDOUT-NEXT: (fun (both (((tuple None None)) 0) (((tuple (con Some _) None)) 1) (((tuple None (con Some _))) 2) (((tuple (con Some _) (con Some _))) 3))) *)
(* CHECK-STDOUT-NEXT: (fun (half (((con Some x)) x))) *)
(* CHECK-STDOUT-NEXT: (val by_case (fn (c (case c (Red "r") (Green "g"))))) *)
(* CHECK-STDOUT-NEXT: (fun (nested (((con Some (con Some x))) x) (((con Some None)) 0) ((None) 0))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 3:13 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 7:11 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 8:23 *)
