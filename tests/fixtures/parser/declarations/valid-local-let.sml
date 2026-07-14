local val a = 1 in val b = a + 1 end
local val a = 1 fun f x = x + a in fun g y = f y end
local local val a = 1 in val b = a end in val c = b end
val sequence = 1; val after_semicolon = 2;
val squared = let fun square x = x * x val n = square 3 in n + 1 end
val destructured = let val (a, b) = (1, 2) in a + b end
val nested = let val a = 1 in let val b = a + 1 in a + b end end
val local_in_let = let local val a = 1 in val b = a end in b end
val mutual = let fun even 0 = true | even n = odd (n - 1) and odd 0 = false | odd n = even (n - 1) in even 4 end
(* CHECK-STDOUT: (local ((val a 1)) ((val b (+ a 1)))) *)
(* CHECK-STDOUT-NEXT: (local ((val a 1) (fun (f ((x) (+ x a))))) ((fun (g ((y) (app f y)))))) *)
(* CHECK-STDOUT-NEXT: (local ((local ((val a 1)) ((val b a)))) ((val c b))) *)
(* CHECK-STDOUT-NEXT: (val sequence 1) *)
(* CHECK-STDOUT-NEXT: (val after_semicolon 2) *)
(* CHECK-STDOUT-NEXT: (val squared (let ((fun (square ((x) {{[(]}}* x x)))) (val n (app square 3))) (+ n 1))) *)
(* CHECK-STDOUT-NEXT: (val destructured (let ((val (tuple a b) (tuple 1 2))) (+ a b))) *)
(* CHECK-STDOUT-NEXT: (val nested (let ((val a 1)) (let ((val b (+ a 1))) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (val local_in_let (let ((local ((val a 1)) ((val b a)))) b)) *)
(* CHECK-STDOUT-NEXT: (val mutual (let ((fun (even ((0) true) ((n) (app odd (- n 1)))) (odd ((0) false) ((n) (app even (- n 1)))))) (app even 4))) *)
