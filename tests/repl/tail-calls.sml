fun loop 0 acc = acc | loop n acc = loop (n - 1) (acc + 1);
val iterations = loop 10000000 0;
fun even 0 = true | even n = odd (n - 1) and odd 0 = false | odd n = even (n - 1);
val parity = even 10000001;
(* CHECK-REPL: val loop = fn : int -> int -> int *)
(* CHECK-REPL-NEXT: val iterations = 10000000 : int *)
(* CHECK-REPL-NEXT: val even = fn : int -> bool *)
(* CHECK-REPL-NEXT: val odd = fn : int -> bool *)
(* CHECK-REPL-NEXT: val parity = false : bool *)
