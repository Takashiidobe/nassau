41;
it + 1;
val cell = ref 0;
cell := 7;
!cell;
fun twice f x = f (f x);
twice (fn n => n + 1) 20;
"a" ^ "b";
let val x = 6 in x * 7 end;
infix 6 ++;
fun a ++ b = a + b;
20 ++ 22;
datatype 'a box = Box of 'a;
Box it;
(* ORACLE-REPL *)
(* CHECK-REPL: val it = 41 : int *)
(* CHECK-REPL-NEXT: val it = 42 : int *)
(* CHECK-REPL-NEXT: val cell = ref 0 : int ref *)
(* CHECK-REPL-NEXT: val it = () : unit *)
(* CHECK-REPL-NEXT: val it = 7 : int *)
(* CHECK-REPL-NEXT: val twice = fn : ('a -> 'a) -> 'a -> 'a *)
(* CHECK-REPL-NEXT: val it = 22 : int *)
(* CHECK-REPL-NEXT: val it = "ab" : string *)
(* CHECK-REPL-NEXT: val it = 42 : int *)
(* CHECK-REPL-NEXT: infix 6 ++ *)
(* CHECK-REPL-NEXT: val ++ = fn : int * int -> int *)
(* CHECK-REPL-NEXT: val it = 42 : int *)
(* CHECK-REPL-NEXT: datatype 'a box = Box of 'a *)
(* CHECK-REPL-NEXT: val it = Box 42 : int box *)
