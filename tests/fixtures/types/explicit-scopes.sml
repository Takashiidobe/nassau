fun 'a outer (x : 'a) =
  let val y : 'a = x
      fun keep (z : 'a) = y
      fun 'a fresh (z : 'a) = z
  in (keep x, fresh 7, fresh true) end
val (a, b, c) = outer "ok"
val _ = print (a ^ Int.toString b ^ Bool.toString c ^ "\n")
val 'a rec id = fn (x : 'a) => x
val _ = print (id "rec\n")
fun (''a, 'b) same (x : ''a, y : ''a, z : 'b) = (x = y, z)
val (yes, s) = same (1, 1, "equal\n")
val _ = if yes then print s else ()
(* CHECK-STDOUT: val outer : 'a -> 'a * int * bool *)
(* CHECK-STDOUT-NEXT: val a : string *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : bool *)
(* CHECK-STDOUT-NEXT: val id : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val same : ''a * ''a * 'b -> bool * 'b *)
(* CHECK-STDOUT-NEXT: val yes : bool *)
(* CHECK-STDOUT-NEXT: val s : string *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: ok7true *)
(* CHECK-RUN-STDOUT-NEXT: rec *)
(* CHECK-RUN-STDOUT-NEXT: equal *)
