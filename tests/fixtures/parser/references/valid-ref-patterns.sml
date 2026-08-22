val r = ref 1
val ref x = r
val ref (a, b) = ref (1, 2)
fun get (ref v) = v
val only = fn ref n => n
val partial = fn ref 0 => 1
val both = fn ref 0 => "zero" | ref _ => "other"
val by_case = case r of ref 0 => 0 | ref n => n
val layered = fn (c as ref v) => (c, v)
val pair = fn (ref a, ref b) => a + b
(* CHECK-STDOUT: (val r (app ref 1)) *)
(* CHECK-STDOUT-NEXT: (val (con ref x) r) *)
(* CHECK-STDOUT-NEXT: (val (con ref (tuple a b)) (app ref (tuple 1 2))) *)
(* CHECK-STDOUT-NEXT: (fun (get (((con ref v)) v))) *)
(* CHECK-STDOUT-NEXT: (val only (fn ((con ref n) n))) *)
(* CHECK-STDOUT-NEXT: (val partial (fn ((con ref 0) 1))) *)
(* CHECK-STDOUT-NEXT: (val both (fn ((con ref 0) "zero") ((con ref _) "other"))) *)
(* CHECK-STDOUT-NEXT: (val by_case (case r ((con ref 0) 0) ((con ref n) n))) *)
(* CHECK-STDOUT-NEXT: (val layered (fn ((as c (con ref v)) (tuple c v)))) *)
(* CHECK-STDOUT-NEXT: (val pair (fn ((tuple (con ref a) (con ref b)) (+ a b)))) *)
(* CHECK-STDERR: warning: match nonexhaustive at 6:15 *)
(* CHECK-RUN-EXIT: 0 *)
