(* SML'97 grammar, declaration: valbind and rec. *)
fun pi n = print (Int.toString n ^ "\n")
val a = 1 and b = 2
val rec f = fn 0 => 0 | n => n + f (n - 1)
val rec g = fn n => if n = 0 then 0 else h (n - 1) and h = fn n => if n = 0 then 1 else g (n - 1)
val _ = pi (a + b + f 3 + h 3)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 9 *)
