exception A
exception B of int
val one = fn A => 1
val two = fn A => 1 | B _ => 2
val three = fn A => 1 | _ => 2
val four = (raise A) handle A => 1
val five = case A of A => 1 | B n => n
val six = fn (A, B n) => n
(* CHECK-STDOUT: (exception (A)) *)
(* CHECK-STDOUT-NEXT: (exception (B int)) *)
(* CHECK-STDOUT-NEXT: (val one (fn (A 1))) *)
(* CHECK-STDOUT-NEXT: (val two (fn (A 1) ((con B _) 2))) *)
(* CHECK-STDOUT-NEXT: (val three (fn (A 1) (_ 2))) *)
(* CHECK-STDOUT-NEXT: (val four (handle (raise A) (A 1))) *)
(* CHECK-STDOUT-NEXT: (val five (case A (A 1) ((con B n) n))) *)
(* CHECK-STDOUT-NEXT: (val six (fn ((tuple A (con B n)) n))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 3:11 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 4:11 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 7:12 *)
(* CHECK-STDERR-NEXT: warning: match nonexhaustive at 8:11 *)
