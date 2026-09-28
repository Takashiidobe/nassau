val xs = [1, 2, 3]
val bound = let val x = 1 val _ = 2 in x + 1; x end
val shadowed = let val x = 1 in let val x = 2 in x end end
val matched = case xs of [] => 0 | x :: rest => x
val numeric_match = case 1 of 0 => "zero" | _ => "other"
val tuple_match = case (1, 2) of (a, b) => a + b
val literal_match = case "a" of "a" => 1 | _ => 0
val anonymous = fn x => x + 1
val typed_argument = fn (x : int) => x
val multiple = fn 0 => "zero" | _ => "other"
(* CHECK-STDOUT: (val xs (list 1 2 3)) *)
(* CHECK-STDOUT-NEXT: (val bound (let ((val x 1) (val _ 2)) (seq (+ x 1) x))) *)
(* CHECK-STDOUT-NEXT: (val shadowed (let ((val x 1)) (let ((val x 2)) x))) *)
(* CHECK-STDOUT-NEXT: (val matched (case xs ((list) 0) ((:: x rest) x))) *)
(* CHECK-STDOUT-NEXT: (val numeric_match (case 1 (0 "zero") (_ "other"))) *)
(* CHECK-STDOUT-NEXT: (val tuple_match (case (tuple 1 2) ((tuple a b) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (val literal_match (case "a" ("a" 1) (_ 0))) *)
(* CHECK-STDOUT-NEXT: (val anonymous (fn (x (+ x 1)))) *)
(* CHECK-STDOUT-NEXT: (val typed_argument (fn ((: x int) x))) *)
(* CHECK-STDOUT-NEXT: (val multiple (fn (0 "zero") (_ "other"))) *)
