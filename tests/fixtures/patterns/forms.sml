(* Every pattern form of tests/fixtures/parser/patterns/valid-syntax.sml. *)
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
fun show n = print (Int.toString n ^ "\n")
fun say s = print (s ^ "\n")
val () = show (layered (1, 2))
val () = show (typed_layered 3)
val () = say (constants 0 ^ ", " ^ constants ~1 ^ ", " ^ constants 7)
val () = show (strings "a" + strings "b" + strings "c")
val () = show (characters #"a" + characters #"b")
val () = show (words 0w5 + words 0wx6)
val () = show (booleans true + booleans false)
val () = show (unit ())
val () = show (tuple (1, 2, 3) + nested_tuple ((1, 2), 3))
val () = show (list [1, 2] + list [1, 2, 3] + list [])
val () = show (cons [1, 2, 3] + cons [4])
val () = show (case typed [5] of [x] => x | _ => 0)
val () = show (option (SOME 4) + option NONE)
val () = show (order LESS + order EQUAL + order GREATER)
val () = show (record + flexible + typed_row + row_layered)
val () = show (numbered (10, 20) + empty_record ())
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: zero, minus one, many *)
(* CHECK-STDOUT-NEXT: 6 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: 1 *)
(* CHECK-STDOUT-NEXT: 1 *)
(* CHECK-STDOUT-NEXT: 12 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: 5 *)
(* CHECK-STDOUT-NEXT: 4 *)
(* CHECK-STDOUT-NEXT: 3 *)
(* CHECK-STDOUT-NEXT: 13 *)
(* CHECK-STDOUT-NEXT: 31 *)
