(* Ten million iterations of tail recursion run in constant stack. *)
fun loop 0 acc = acc
  | loop n acc = loop (n - 1) (acc + 1)
val _ = print (Int.toString (loop 10000000 0) ^ "\n")
(* Mutually recursive tail calls, too. *)
fun ping 0 = "ping"
  | ping n = pong (n - 1)
and pong 0 = "pong"
  | pong n = ping (n - 1)
val _ = print (ping 10000001 ^ "\n")
(* And calls through closures in tail position. *)
fun repeat f 0 x = x
  | repeat f n x = repeat f (n - 1) (f x)
val _ = print (Int.toString (repeat (fn x => x + 3) 10000000 0) ^ "\n")
val self = ref (fn (n : int) => n)
val _ = self := (fn 0 => 0 | n => (!self) (n - 1))
val _ = print (Int.toString ((!self) 10000000) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 10000000 *)
(* CHECK-STDOUT-NEXT: pong *)
(* CHECK-STDOUT-NEXT: 30000000 *)
(* CHECK-STDOUT-NEXT: 0 *)
