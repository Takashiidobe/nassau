(* GC-PLAN: MarkSweep *)
(* GC-HEAP: 8m *)
(* GC-STRESS: 1 *)
fun grow 0 text = text | grow n text = grow (n - 1) (text ^ text);
val payload = grow 18 "x";
fun original () = size payload;
val saved = original;
fun original () = 0;
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val payload = grow 18 "y";
val retained = saved ();
val current = size payload;
val () = if retained = 262144 andalso current = 262144 then print "history reclaimed, closure retained\n" else raise Fail "history";
(* CHECK-REPL: val retained = 262144 : int *)
(* CHECK-REPL-NEXT: val current = 262144 : int *)
(* CHECK-REPL-NEXT: history reclaimed, closure retained *)
(* CHECK-STDOUT: history reclaimed, closure retained *)
