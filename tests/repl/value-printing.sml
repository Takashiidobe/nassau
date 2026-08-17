datatype 'a tree = Leaf of 'a | Branch of 'a tree * 'a tree | Tip;
val tree = Branch (Leaf "hello", Branch (Tip, Leaf "world"));
val leaves = [Leaf (SOME 3), Tip, Leaf NONE];
datatype colour = Red | Green | Blue;
val colours = [Red,Green,Blue];
datatype single = Single of int;
val single = Single 42;
exception Payload of int list;
val ex = Payload [1,2];
exception Alias = Payload;
val alias = Alias [3];
exception Payload of string;
val newer = Payload "new";
val older = ex;
val builtin = Fail "bad";
val controls = "\001\127\255\n\r\t\b\f";
val ch = #"\255";
val spaced = SOME "two words";
fun range n = if n = 25 then [] else n :: range (n + 1);
val long = range 0;
datatype loop = Loop of loop ref | End;
val cell = ref End;
val () = cell := Loop cell;
val cycle = !cell;
abstype hidden = Hidden of int with val hidden = Hidden 3 end;
(* CHECK-REPL: val tree = Branch (Leaf "hello",Branch (Tip,Leaf "world")) : string tree *)
(* CHECK-REPL: val leaves = {{[[]}}Leaf (SOME 3),Tip,Leaf NONE] : int option tree list *)
(* CHECK-REPL: val colours = {{[[]}}Red,Green,Blue] : colour list *)
(* CHECK-REPL: val single = Single 42 : single *)
(* CHECK-REPL: val ex = Payload {{[[]}}1,2] : exn *)
(* CHECK-REPL: val alias = Payload {{[[]}}3] : exn *)
(* CHECK-REPL: val newer = Payload "new" : exn *)
(* CHECK-REPL: val older = Payload {{[[]}}1,2] : exn *)
(* CHECK-REPL: val builtin = Fail "bad" : exn *)
(* CHECK-REPL: val controls = "\^A\127\255\n\r\t\b\f" : string *)
(* CHECK-REPL: val ch = #"\255" : char *)
(* CHECK-REPL: val spaced = SOME "two words" : string option *)
(* CHECK-REPL: val range = fn : int -> int list *)
(* CHECK-REPL: val long = {{[[]}}0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,...] : int list *)
(* CHECK-REPL: val cell = ref End : loop ref *)
(* CHECK-REPL: val cycle = Loop (ref (Loop (ref (Loop (ref (Loop (ref (Loop (ref #))))))))) : loop *)
(* CHECK-REPL: val hidden = - : hidden *)
