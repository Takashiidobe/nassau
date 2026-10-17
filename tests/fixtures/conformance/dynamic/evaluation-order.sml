val state = ref 0
fun tick n = (state := !state * 10 + n; n)
fun make n = (tick n; fn x => x)
val _ = (make 1) (tick 2)
val _ = (tick 3, tick 4)
val _ = {b = tick 5, a = tick 6}
val _ = [tick 7, tick 8]
val _ = false andalso (tick 9 = 9)
val _ = true orelse (tick 9 = 9)
val _ = print (Int.toString (!state) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 12345678 *)
