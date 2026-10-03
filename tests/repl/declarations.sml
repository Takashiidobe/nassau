type pair = int * int;
datatype token = A | B of int;
exception E of token;
signature S = sig type t val x : t val f : int -> int end;
structure X = struct type t = int val x = 3 fun f n = n + x end;
functor F (Y : S) = struct val answer = Y.f 4 end;
structure R = F (X);
open X;
infix 6 ++;
fun a ++ b = a + b;
val n = 1 ++ 2;
datatype copied = datatype token;
val copied = B 7;
val raised = E copied;
val caught = (raise raised) handle E (B n) => n | E A => 0;
nonfix ++;
val applied = ++ (caught, R.answer);
val recursive = let fun sum [] = 0 | sum (n::ns) = n + sum ns in sum end;
val result = recursive [applied, X.f 1];
(* ORACLE-REPL *)
(* CHECK-REPL: type pair = int * int *)
(* CHECK-REPL: datatype token = A | B of int *)
(* CHECK-REPL: exception E of token *)
(* CHECK-REPL: signature S = sig val f: int -> int type t val x: t end *)
(* CHECK-REPL: structure X: sig val f: int -> int eqtype t val x: int end *)
(* CHECK-REPL: functor F (Y: S): sig val answer: int end *)
(* CHECK-REPL: structure R: sig val answer: int end *)
(* CHECK-REPL: eqtype t *)
(* CHECK-REPL: val x = 3 : int *)
(* CHECK-REPL: val f = fn : int -> int *)
(* CHECK-REPL: infix 6 ++ *)
(* CHECK-REPL: val ++ = fn : int * int -> int *)
(* CHECK-REPL: val n = 3 : int *)
(* CHECK-REPL: datatype copied = A | B of int *)
(* CHECK-REPL: val copied = B 7 : copied *)
(* CHECK-REPL: val raised = E (B 7) : exn *)
(* CHECK-REPL: val caught = 7 : int *)
(* CHECK-REPL: nonfix ++ *)
(* CHECK-REPL: val applied = 14 : int *)
(* CHECK-REPL: val recursive = fn : int list -> int *)
(* CHECK-REPL: val result = 18 : int *)
