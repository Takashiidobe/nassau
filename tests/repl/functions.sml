fun fact 0 = 1 | fact n = n * fact (n - 1);
val x = fact 10;
fun swap (a, b) = (b, a);
val pair = swap ("one", 1);
val r = {name = "x", size = size "four"};
(* CHECK-REPL: val fact = fn : int -> int *)
(* CHECK-REPL-NEXT: val x = 3628800 : int *)
(* CHECK-REPL-NEXT: val swap = fn : 'a * 'b -> 'b * 'a *)
(* CHECK-REPL-NEXT: val pair = (1,"one") : int * string *)
(* CHECK-REPL-NEXT: val r = {name="x",size=4} : {name:string, size:int} *)
