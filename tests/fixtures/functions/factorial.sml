(* A recursive function called from top-level bindings. *)
fun factorial 0 = 1
  | factorial n = n * factorial (n - 1)
val _ = print (Int.toString (factorial 10) ^ "\n")
val _ = print (Int.toString (factorial 12) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3628800 *)
(* CHECK-STDOUT-NEXT: 479001600 *)
