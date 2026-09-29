fun fib n = if n < 2 then n else fib (n - 1) + fib (n - 2)
val _ = print ("fib 20 = " ^ Int.toString (fib 20) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: fib 20 = 6765 *)
