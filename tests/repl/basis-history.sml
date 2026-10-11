(* GC-PLAN: MarkSweep *)
(* GC-HEAP: 8m *)
(* GC-STRESS: 1 *)
val saved = String.map;
structure String = struct val hidden = 0 end;
val result = saved Char.toUpper "basis";
val () = print (result ^ "\n");
(* CHECK-REPL: val result = "BASIS" : string *)
(* CHECK-REPL-NEXT: BASIS *)
(* CHECK-STDOUT: BASIS *)
