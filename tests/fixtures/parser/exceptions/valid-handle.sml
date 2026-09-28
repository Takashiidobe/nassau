exception Bad of int
exception Worse
val a = (raise Bad 1) handle Bad n => n
val b = (raise Worse) handle Bad _ => 0 | Worse => 1
val c = (raise Worse) handle _ => 2
val d = (raise Fail "x") handle Fail s => size s | Div => 0
val e = 1 handle Bad n => n
val f = (raise Bad 2) handle Bad 1 => 1 | Bad n => n
fun g x = if x then raise Worse else 1
val h = g true handle Worse => 0
(* CHECK-STDOUT: (exception (Bad int)) *)
(* CHECK-STDOUT-NEXT: (exception (Worse)) *)
(* CHECK-STDOUT-NEXT: (val a (handle (raise (app Bad 1)) ((con Bad n) n))) *)
(* CHECK-STDOUT-NEXT: (val b (handle (raise Worse) ((con Bad _) 0) (Worse 1))) *)
(* CHECK-STDOUT-NEXT: (val c (handle (raise Worse) (_ 2))) *)
(* CHECK-STDOUT-NEXT: (val d (handle (raise (app Fail "x")) ((con Fail s) (app size s)) (Div 0))) *)
(* CHECK-STDOUT-NEXT: (val e (handle 1 ((con Bad n) n))) *)
(* CHECK-STDOUT-NEXT: (val f (handle (raise (app Bad 2)) ((con Bad 1) 1) ((con Bad n) n))) *)
(* CHECK-STDOUT-NEXT: (fun (g ((x) (if x (raise Worse) 1)))) *)
(* CHECK-STDOUT-NEXT: (val h (handle (app g true) (Worse 0))) *)
