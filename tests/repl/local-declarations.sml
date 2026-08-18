local datatype hidden = Hidden of int exception Local of hidden in fun make n = Local (Hidden n) end;
val one = make 1;
val two = make 2;
local type number = int datatype 'a box = Box of 'a withtype numbers = number list in val localValue = Box [1,2] end;
val later = localValue;
val rec even = fn 0 => true | n => odd (n - 1) and odd = fn 0 => false | n => even (n - 1);
val parity = even 6;
fun fresh n = let exception Fresh of int in (Fresh n, fn e => (raise e) handle Fresh n => n | _ => ~1) end;
val (first, catchFirst) = fresh 4;
val (second, catchSecond) = fresh 5;
val handled = (catchFirst first, catchFirst second, catchSecond second);
(* ORACLE-VALUES *)
(* CHECK-REPL: val make = fn : int -> exn *)
(* CHECK-REPL: val one = Local (Hidden 1) : exn *)
(* CHECK-REPL: val two = Local (Hidden 2) : exn *)
(* CHECK-REPL: val localValue = Box {{[[]}}1,2] : int list box *)
(* CHECK-REPL: val later = Box {{[[]}}1,2] : int list box *)
(* CHECK-REPL: val even = fn : int -> bool *)
(* CHECK-REPL: val odd = fn : int -> bool *)
(* CHECK-REPL: val parity = true : bool *)
(* CHECK-REPL: val fresh = fn : int -> exn * (exn -> int) *)
(* CHECK-REPL: val first = Fresh 4 : exn *)
(* CHECK-REPL: val catchFirst = fn : exn -> int *)
(* CHECK-REPL: val second = Fresh 5 : exn *)
(* CHECK-REPL: val catchSecond = fn : exn -> int *)
(* CHECK-REPL: val handled = (4,~1,5) : int * int * int *)
