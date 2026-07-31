val a = SOME 3;
val b = SOME (SOME ~2);
val c : int option = NONE;
val d = [LESS, EQUAL, GREATER];
val w = 0wx1F;
fun get (SOME x) = x | get NONE = 0;
val e = get a + get c;
fun firstTwo (x :: y :: _) = SOME (x, y) | firstTwo _ = NONE;
val f = firstTwo [1, 2, 3];
val g = case f of SOME (x, y) => x + y | NONE => 0;
(* CHECK-REPL: val a = SOME 3 : int option *)
(* CHECK-REPL-NEXT: val b = SOME (SOME ~2) : int option option *)
(* CHECK-REPL-NEXT: val c = NONE : int option *)
(* CHECK-REPL-NEXT: val d = [LESS,EQUAL,GREATER] : order list *)
(* CHECK-REPL-NEXT: val w = 0wx1F : word *)
(* CHECK-REPL-NEXT: val get = fn : int option -> int *)
(* CHECK-REPL-NEXT: val e = 3 : int *)
(* CHECK-REPL-NEXT: val firstTwo = fn : 'a list -> ('a * 'a) option *)
(* CHECK-REPL-NEXT: val f = SOME (1,2) : (int * int) option *)
(* CHECK-REPL-NEXT: val g = 3 : int *)
