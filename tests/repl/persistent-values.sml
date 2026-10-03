datatype 'a box = Box of 'a | Empty;
fun make n = let val cell = ref n in fn x => (cell := !cell + x; !cell) end;
val add = make 10;
val boxed = Box [add];
fun run (Box [f]) = f 2 | run _ = 0;
val first = run boxed;
val second = add 3;
exception Saved of int;
val saved = Saved second;
fun catch e = (raise e) handle Saved n => n;
val caught = catch saved;
val old = add;
val add = fn n => n * 2;
val preserved = old 4;
val changed = add 4;
val () = print (Int.toString first ^ "," ^ Int.toString second ^ "," ^ Int.toString caught ^ "," ^ Int.toString preserved ^ "," ^ Int.toString changed ^ "\n");
(* CHECK-REPL: val first = 12 : int *)
(* CHECK-REPL-NEXT: val second = 15 : int *)
(* CHECK-REPL: val caught = 15 : int *)
(* CHECK-REPL: val preserved = 19 : int *)
(* CHECK-REPL-NEXT: val changed = 8 : int *)
(* CHECK-REPL-NEXT: 12,15,15,19,8 *)
(* CHECK-STDOUT: 12,15,15,19,8 *)
