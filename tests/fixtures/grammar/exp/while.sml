(* SML'97 grammar, expression: while. *)
fun pi n = print (Int.toString n ^ "\n")
val i = ref 0 val s = ref 0
val _ = while !i < 5 do (s := !s + !i; i := !i + 1)
val _ = pi (!s)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 10 *)
