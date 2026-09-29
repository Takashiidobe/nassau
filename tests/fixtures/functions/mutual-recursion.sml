(* Mutually recursive functions declared with and. *)
fun isEven 0 = true
  | isEven n = isOdd (n - 1)
and isOdd 0 = false
  | isOdd n = isEven (n - 1)
val _ = print (if isEven 10 then "10 is even\n" else "10 is odd\n")
val _ = print (if isOdd 7 then "7 is odd\n" else "7 is even\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 10 is even *)
(* CHECK-STDOUT-NEXT: 7 is odd *)
