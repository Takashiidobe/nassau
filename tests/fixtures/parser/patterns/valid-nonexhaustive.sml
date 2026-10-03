val literal = fn 0 => "zero"
val missing_false = fn true => 1
val missing_nil = fn x :: xs => x
val missing_cons = fn [] => 0
val short_list = fn [x] => x | [] => 0
val pair = fn (true, true) => 1 | (false, false) => 2
val option = fn SOME x => x
val order = fn LESS => 0 | EQUAL => 1
val strings = fn "a" => 1 | "b" => 2
val nested = fn SOME true => 1 | NONE => 2
val case_expression = case 0 of 0 => 1
val outer = fn x => fn 0 => x
(* CHECK-STDOUT: (val literal (fn (0 "zero"))) *)
(* CHECK-STDOUT-NEXT: (val missing_false (fn (true 1))) *)
(* CHECK-STDOUT-NEXT: (val missing_nil (fn ((:: x xs) x))) *)
(* CHECK-STDOUT-NEXT: (val missing_cons (fn ((list) 0))) *)
(* CHECK-STDOUT-NEXT: (val short_list (fn ((list x) x) ((list) 0))) *)
(* CHECK-STDOUT-NEXT: (val pair (fn ((tuple true true) 1) ((tuple false false) 2))) *)
(* CHECK-STDOUT-NEXT: (val option (fn ((con SOME x) x))) *)
(* CHECK-STDOUT-NEXT: (val order (fn (LESS 0) (EQUAL 1))) *)
(* CHECK-STDOUT-NEXT: (val strings (fn ("a" 1) ("b" 2))) *)
(* CHECK-STDOUT-NEXT: (val nested (fn ((con SOME true) 1) (NONE 2))) *)
(* CHECK-STDOUT-NEXT: (val case_expression (case 0 (0 1))) *)
(* CHECK-STDOUT-NEXT: (val outer (fn (x (fn (0 x))))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 1:15 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 2:21 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 3:19 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 4:20 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 5:18 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 6:12 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 7:14 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 8:13 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 9:15 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 10:14 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 11:23 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 12:21 *)
(* CHECK-RUN-EXIT: 0 *)
