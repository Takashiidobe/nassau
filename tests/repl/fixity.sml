infixr 5 ++;
fun a ++ b = a * 10 + b;
val right = 1 ++ 2 ++ 3;
local infix 7 ** fun a ** b = a * b in infix 6 ++ fun a ++ b = a + b end;
val left = 1 ++ 2 ++ 3;
structure Scoped = struct nonfix ++ val f = ++ end;
val after = 4 ++ 5;
nonfix ++;
val prefix = ++ (6,7);
(* ORACLE-REPL *)
(* CHECK-REPL: infixr 5 ++ *)
(* CHECK-REPL: val ++ = fn : int * int -> int *)
(* CHECK-REPL: val right = 33 : int *)
(* CHECK-REPL: infix 6 ++ *)
(* CHECK-REPL: val ++ = fn : int * int -> int *)
(* CHECK-REPL: val left = 6 : int *)
(* CHECK-REPL: structure Scoped: sig val f: int * int -> int end *)
(* CHECK-REPL: val after = 9 : int *)
(* CHECK-REPL: nonfix ++ *)
(* CHECK-REPL: val prefix = 13 : int *)
