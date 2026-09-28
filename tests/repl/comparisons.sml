val greater = 3 > 2;
val less = 1 < 2;
val equal = 8 = 8;
val realGreater = 2.0 > 1.0;
val realLess = 1.0 < 2.0;
val integerGreaterOrEqual = 3 >= 3;
val integerLessOrEqual = 3 <= 3;
val integerNotEqual = 3 <> 4;
val realGreaterOrEqual = 2.0 >= 2.0;
val realLessOrEqual = 2.0 <= 2.0;
val yes = 1 < 2;
val no = 3 < 2;
val booleanNotEqual = yes <> no;
val trueLiteral = true;
val falseLiteral = false;
val trueBranch = if true then 10 else 20;
val falseBranch = if false then 10 else 20;
val comparisonBranch = if 2 >= 2 then 30 else 40;
val nestedBranch = if true then if false then 50 else 60 else 70;
val booleanBranch = if false then true else false;
val realBranch = if 1 < 2 then 1.5 else 2.5;
(* CHECK-REPL: val greater = true : bool *)
(* CHECK-REPL-NEXT: val less = true : bool *)
(* CHECK-REPL-NEXT: val equal = true : bool *)
(* CHECK-REPL-NEXT: val realGreater = true : bool *)
(* CHECK-REPL-NEXT: val realLess = true : bool *)
(* CHECK-REPL-NEXT: val integerGreaterOrEqual = true : bool *)
(* CHECK-REPL-NEXT: val integerLessOrEqual = true : bool *)
(* CHECK-REPL-NEXT: val integerNotEqual = true : bool *)
(* CHECK-REPL-NEXT: val realGreaterOrEqual = true : bool *)
(* CHECK-REPL-NEXT: val realLessOrEqual = true : bool *)
(* CHECK-REPL-NEXT: val yes = true : bool *)
(* CHECK-REPL-NEXT: val no = false : bool *)
(* CHECK-REPL-NEXT: val booleanNotEqual = true : bool *)
(* CHECK-REPL-NEXT: val trueLiteral = true : bool *)
(* CHECK-REPL-NEXT: val falseLiteral = false : bool *)
(* CHECK-REPL-NEXT: val trueBranch = 10 : int *)
(* CHECK-REPL-NEXT: val falseBranch = 20 : int *)
(* CHECK-REPL-NEXT: val comparisonBranch = 30 : int *)
(* CHECK-REPL-NEXT: val nestedBranch = 60 : int *)
(* CHECK-REPL-NEXT: val booleanBranch = false : bool *)
(* CHECK-REPL-NEXT: val realBranch = 1.5 : real *)
