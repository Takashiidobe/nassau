val booleans = fn true => 1 | false => 0
val wildcard = fn 0 => "zero" | _ => "other"
val variable = fn 0 => "zero" | n => "other"
val list = fn [] => 0 | x :: xs => x
val list_lengths = fn [] => 0 | [x] => 1 | x :: y :: rest => 2
val pair = fn (true, true) => 1 | (true, false) => 2 | (false, _) => 3
val nested = fn (SOME true) => 1 | (SOME false) => 2 | NONE => 3
val option = fn SOME x => x | NONE => 0
val order = fn LESS => 0 | EQUAL => 1 | GREATER => 2
val tuple = fn (a, b) => a + b
val unit = fn () => 1
val layered = fn all as (a, b) => a + b
val case_expression = case (1, true) of (_, true) => 1 | (n, false) => n
val handler = (1 div 0) handle Div => 0
(* CHECK-STDOUT: (val booleans (fn (true 1) (false 0))) *)
(* CHECK-STDOUT-NEXT: (val wildcard (fn (0 "zero") (_ "other"))) *)
(* CHECK-STDOUT-NEXT: (val variable (fn (0 "zero") (n "other"))) *)
(* CHECK-STDOUT-NEXT: (val list (fn ((list) 0) ((:: x xs) x))) *)
(* CHECK-STDOUT-NEXT: (val list_lengths (fn ((list) 0) ((list x) 1) ((:: x (:: y rest)) 2))) *)
(* CHECK-STDOUT-NEXT: (val pair (fn ((tuple true true) 1) ((tuple true false) 2) ((tuple false _) 3))) *)
(* CHECK-STDOUT-NEXT: (val nested (fn ((con SOME true) 1) ((con SOME false) 2) (NONE 3))) *)
(* CHECK-STDOUT-NEXT: (val option (fn ((con SOME x) x) (NONE 0))) *)
(* CHECK-STDOUT-NEXT: (val order (fn (LESS 0) (EQUAL 1) (GREATER 2))) *)
(* CHECK-STDOUT-NEXT: (val tuple (fn ((tuple a b) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (val unit (fn (() 1))) *)
(* CHECK-STDOUT-NEXT: (val layered (fn ((as all (tuple a b)) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (val case_expression (case (tuple 1 true) ((tuple _ true) 1) ((tuple n false) n))) *)
(* CHECK-STDOUT-NEXT: (val handler (handle (div 1 0) (Div 0))) *)
