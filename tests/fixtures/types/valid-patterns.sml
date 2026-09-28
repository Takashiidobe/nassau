fun head_or_zero [] = 0 | head_or_zero (x :: _) = x
fun unwrap (SOME x) = x | unwrap NONE = 0
fun kind 0 = "zero" | kind 1 = "one" | kind _ = "many"
fun is_a #"a" = true | is_a _ = false
fun pick {a = x, b = _} = x
fun third (_, _, z) = z
fun describe [] = "none" | describe [_] = "one" | describe _ = "many"
fun layered (l as x :: _) = (x, l) | layered [] = (0, [])
val by_case = fn n => case n of 0 => "zero" | _ => "other"
fun cmp (a, b) = case (if a < b then LESS else if a = b then EQUAL else GREATER) of LESS => ~1 | EQUAL => 0 | GREATER => 1
(* CHECK-STDOUT: val head_or_zero : int list -> int *)
(* CHECK-STDOUT-NEXT: val unwrap : int option -> int *)
(* CHECK-STDOUT-NEXT: val kind : int -> string *)
(* CHECK-STDOUT-NEXT: val is_a : char -> bool *)
(* CHECK-STDOUT-NEXT: val pick : {a:'a, b:'b} -> 'a *)
(* CHECK-STDOUT-NEXT: val third : 'a * 'b * 'c -> 'c *)
(* CHECK-STDOUT-NEXT: val describe : 'a list -> string *)
(* CHECK-STDOUT-NEXT: val layered : int list -> int * int list *)
(* CHECK-STDOUT-NEXT: val by_case : int -> string *)
(* CHECK-STDOUT-NEXT: val cmp : int * int -> int *)
