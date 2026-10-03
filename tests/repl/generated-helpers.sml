val prefix = "12345678";
val joined = prefix ^ "abcdefgh";
val equal = joined = ("1234" ^ "5678abcdefgh");
val minimum = Int.toString ~1073741824;
val maximum = Int.toString 1073741823;
val zero = Int.toString 0;
val empty = "" ^ "";
val equalEmpty = empty = "";
val different = ("a\000" ^ "b") <> ("a\000" ^ "c");
fun same (x, y) = x = y;
val nested = same ([SOME (joined, [1, 2])], [SOME (prefix ^ "abcdefgh", [1, 2])]);
(* CHECK-REPL: val prefix = "12345678" : string *)
(* CHECK-REPL-NEXT: val joined = "12345678abcdefgh" : string *)
(* CHECK-REPL-NEXT: val equal = true : bool *)
(* CHECK-REPL-NEXT: val minimum = "~1073741824" : string *)
(* CHECK-REPL-NEXT: val maximum = "1073741823" : string *)
(* CHECK-REPL-NEXT: val zero = "0" : string *)
(* CHECK-REPL-NEXT: val empty = "" : string *)
(* CHECK-REPL-NEXT: val equalEmpty = true : bool *)
(* CHECK-REPL-NEXT: val different = true : bool *)
(* CHECK-REPL-NEXT: val same = fn : ''a * ''a -> bool *)
(* CHECK-REPL-NEXT: val nested = true : bool *)
