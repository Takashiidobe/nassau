fun add x y = x + y
fun addr (x : real) y = x + y
fun half x = x / 2.0
fun square x = x * x
fun negate x = 0 - x
fun below x y = x < y
fun order (a, b) = if a < b then a else b
fun over x = x > "m"
fun rem x = x mod 3
val real_sum = 1.5 + 2.5
val words = 0w3 + 0w4
val mixed = fn x => x + 1.0
fun sum_squares (a, b) = a * a + b * b
val default_lambda = fn x => x + x
val chars = #"a" < #"b"
(* RUNTIME-SKIP: word arithmetic and comparisons are not supported by code generation yet *)
(* CHECK-STDOUT: val add : int -> int -> int *)
(* CHECK-STDOUT-NEXT: val addr : real -> real -> real *)
(* CHECK-STDOUT-NEXT: val half : real -> real *)
(* CHECK-STDOUT-NEXT: val square : int -> int *)
(* CHECK-STDOUT-NEXT: val negate : int -> int *)
(* CHECK-STDOUT-NEXT: val below : int -> int -> bool *)
(* CHECK-STDOUT-NEXT: val order : int * int -> int *)
(* CHECK-STDOUT-NEXT: val over : string -> bool *)
(* CHECK-STDOUT-NEXT: val rem : int -> int *)
(* CHECK-STDOUT-NEXT: val real_sum : real *)
(* CHECK-STDOUT-NEXT: val words : word *)
(* CHECK-STDOUT-NEXT: val mixed : real -> real *)
(* CHECK-STDOUT-NEXT: val sum_squares : int * int -> int *)
(* CHECK-STDOUT-NEXT: val default_lambda : int -> int *)
(* CHECK-STDOUT-NEXT: val chars : bool *)
(* CHECK-RUN-ERR: × word arithmetic and comparisons are not supported by code generation yet *)
(* CHECK-RUN-ERR: :11:13] *)
