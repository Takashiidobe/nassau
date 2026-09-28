val layered = fn (all as (first, second)) => first + second
val typed_layered = fn (x : int as y) => y
val constants = fn 0 => "zero" | ~1 => "minus one" | _ => "many"
val strings = fn "a" => 1 | "b" => 2 | _ => 3
val characters = fn #"a" => 1 | _ => 2
val words = fn 0w5 => 1 | _ => 2
val booleans = fn true => 1 | false => 0
val unit = fn () => 1
val tuple = fn (a, b, c) => a + b + c
val nested_tuple = fn ((a, b), c) => a + b + c
val list = fn [a, b] => a + b | _ => 0
val cons = fn x :: y :: rest => x + y | _ => 0
val typed = fn (x : int list) => x
val option = fn SOME x => x | NONE => 0
val order = fn LESS => 0 | EQUAL => 1 | GREATER => 2
val record = (fn {a, b = c} => a + c) {a = 1, b = 2}
val flexible = (fn {a, ...} => a) {a = 1, b = 2}
val typed_row = (fn {a : int, b} => a + b) {a = 1, b = 2}
val row_layered = (fn {a, b as (c, d)} => a + c + d) {a = 1, b = (2, 3)}
val numbered = fn {1 = a, 2 = b} => a + b
val empty_record = fn {} => 1
(* CHECK-STDOUT: (val layered (fn ((as all (tuple first second)) (+ first second)))) *)
(* CHECK-STDOUT-NEXT: (val typed_layered (fn ((as x int y) y))) *)
(* CHECK-STDOUT-NEXT: (val constants (fn (0 "zero") (-1 "minus one") (_ "many"))) *)
(* CHECK-STDOUT-NEXT: (val strings (fn ("a" 1) ("b" 2) (_ 3))) *)
(* CHECK-STDOUT-NEXT: (val characters (fn (#"a" 1) (_ 2))) *)
(* CHECK-STDOUT-NEXT: (val words (fn (0w5 1) (_ 2))) *)
(* CHECK-STDOUT-NEXT: (val booleans (fn (true 1) (false 0))) *)
(* CHECK-STDOUT-NEXT: (val unit (fn (() 1))) *)
(* CHECK-STDOUT-NEXT: (val tuple (fn ((tuple a b c) (+ (+ a b) c)))) *)
(* CHECK-STDOUT-NEXT: (val nested_tuple (fn ((tuple (tuple a b) c) (+ (+ a b) c)))) *)
(* CHECK-STDOUT-NEXT: (val list (fn ((list a b) (+ a b)) (_ 0))) *)
(* CHECK-STDOUT-NEXT: (val cons (fn ((:: x (:: y rest)) (+ x y)) (_ 0))) *)
(* CHECK-STDOUT-NEXT: (val typed (fn ((: x (tycon list int)) x))) *)
(* CHECK-STDOUT-NEXT: (val option (fn ((con SOME x) x) (NONE 0))) *)
(* CHECK-STDOUT-NEXT: (val order (fn (LESS 0) (EQUAL 1) (GREATER 2))) *)
(* CHECK-STDOUT-NEXT: (val record (app (fn ((record (a a) (b c)) (+ a c))) (record (a 1) (b 2)))) *)
(* CHECK-STDOUT-NEXT: (val flexible (app (fn ((record (a a) ...) a)) (record (a 1) (b 2)))) *)
(* CHECK-STDOUT-NEXT: (val typed_row (app (fn ((record (a (: a int)) (b b)) (+ a b))) (record (a 1) (b 2)))) *)
(* CHECK-STDOUT-NEXT: (val row_layered (app (fn ((record (a a) (b (as b (tuple c d)))) (+ (+ a c) d))) (record (a 1) (b (tuple 2 3))))) *)
(* CHECK-STDOUT-NEXT: (val numbered (fn ((record (1 a) (2 b)) (+ a b)))) *)
(* CHECK-STDOUT-NEXT: (val empty_record (fn (() 1))) *)
