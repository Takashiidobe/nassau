fun op ++ (a, b) = a + b
val inside_let = let infix 6 ++ in 1 ++ 2 * 3 end
val after_let = ++ (1, 2)
local infix 6 ++ in val inside_local = 1 ++ 2 end
val after_local = ++ (3, 4)
local infix 6 ++ val private = 1 ++ 2 in val public = 3 ++ 4 end
val after_both = ++ (5, 6)
(* CHECK-STDOUT: (fun (++ (((tuple a b)) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (val inside_let (let ((infix 6 ++)) (++ 1 {{[(]}}* 2 3)))) *)
(* CHECK-STDOUT-NEXT: (val after_let (app ++ (tuple 1 2))) *)
(* CHECK-STDOUT-NEXT: (local ((infix 6 ++)) ((val inside_local (++ 1 2)))) *)
(* CHECK-STDOUT-NEXT: (val after_local (app ++ (tuple 3 4))) *)
(* CHECK-STDOUT-NEXT: (local ((infix 6 ++) (val private (++ 1 2))) ((val public (++ 3 4)))) *)
(* CHECK-STDOUT-NEXT: (val after_both (app ++ (tuple 5 6))) *)
(* CHECK-RUN-EXIT: 0 *)
