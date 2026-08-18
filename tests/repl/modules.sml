datatype 'a box = Box of 'a;
signature S = sig type t val x : t datatype d = A | B of int exception E of string structure Inner : sig val n : int end end;
structure X = struct datatype d = A | B of int type t = int val x = 3 val x = 4 exception E of string structure Inner = struct val n = 9 end end;
structure Abstract :> sig type t val value : t end = struct type t = int val value = 3 end;
val hidden = Abstract.value;
val boxed = X.B 3;
val ex = X.E "x";
open X;
val it = x;
(* CHECK-REPL: datatype 'a box = Box of 'a *)
(* CHECK-REPL: signature S = sig exception E of string structure Inner: sig val n: int end datatype d = A | B of int type t val x: t end *)
(* CHECK-REPL: structure X: sig exception E of string structure Inner: sig val n: int end datatype d = A | B of int eqtype t val x: int end *)
(* CHECK-REPL: structure Abstract: sig type t val value: t end *)
(* CHECK-REPL: val hidden = - : Abstract.t *)
(* CHECK-REPL: val boxed = B 3 : X.d *)
(* CHECK-REPL: val ex = E "x" : exn *)
(* CHECK-REPL: datatype d = A | B of int *)
(* CHECK-REPL: eqtype t *)
(* CHECK-REPL: structure Inner: sig val n: int end *)
(* CHECK-REPL: val x = 4 : int *)
(* CHECK-REPL: exception E of string *)
(* CHECK-REPL: val it = 4 : int *)
