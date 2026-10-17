val 'a f = fn (x : 'a) => x and g = fn (y : 'a) => y
val _ = print (f "val" ^ g "-and\n")
fun 'a left (x : 'a) = right x and right (y : 'a) = y
val _ = print (left "fun-and\n")
fun 'a unused x = x
val _ = print (unused "unused\n")
(* CHECK-STDOUT: val f : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val g : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val left : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val right : 'a -> 'a *)
(* CHECK-STDOUT-NEXT: val unused : 'a -> 'a *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: val-and *)
(* CHECK-RUN-STDOUT-NEXT: fun-and *)
(* CHECK-RUN-STDOUT-NEXT: unused *)
