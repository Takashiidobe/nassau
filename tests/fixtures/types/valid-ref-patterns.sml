val r = ref 1
val ref x = r
val ref (a, b) = ref (1, "s")
fun get (ref v) = v
val only = fn ref n => n
val zero = fn ref 0 => "zero" | ref _ => "other"
val by_case = case r of ref 0 => 0 | ref n => n
val layered = fn (c as ref v) => (c, v)
val pair = fn (ref a, ref b) => a + b
val poly = fn (ref x) => x
(* CHECK-STDOUT: val r : int ref *)
(* CHECK-STDOUT-NEXT: val x : int *)
(* CHECK-STDOUT-NEXT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : string *)
(* CHECK-STDOUT-NEXT: val get : 'a ref -> 'a *)
(* CHECK-STDOUT-NEXT: val only : 'a ref -> 'a *)
(* CHECK-STDOUT-NEXT: val zero : int ref -> string *)
(* CHECK-STDOUT-NEXT: val by_case : int *)
(* CHECK-STDOUT-NEXT: val layered : 'a ref -> 'a ref * 'a *)
(* CHECK-STDOUT-NEXT: val pair : int ref * int ref -> int *)
(* CHECK-STDOUT-NEXT: val poly : 'a ref -> 'a *)
(* CHECK-RUN-EXIT: 0 *)
