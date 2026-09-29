fun swap (a, b) = (b, a)
fun add3 (a, b, c) = a + b + c
val (x, y) = swap (1, 2)
val nested = ((1, "one"), (2, "two"))
val ((n, s), (m, t)) = nested
val () = print (Int.toString x ^ " " ^ Int.toString y ^ "\n")
val () = print (s ^ t ^ " " ^ Int.toString (add3 (n, m, 3)) ^ "\n")
val u = ()
fun unitArg () = print "unit\n"
val () = unitArg u
fun pairs [] = [] | pairs (x :: xs) = (x, x * x) :: pairs xs
fun sumSquares [] = 0 | sumSquares ((_, sq) :: rest) = sq + sumSquares rest
val () = print (Int.toString (sumSquares (pairs [1, 2, 3])) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 1 *)
(* CHECK-STDOUT-NEXT: onetwo 6 *)
(* CHECK-STDOUT-NEXT: unit *)
(* CHECK-STDOUT-NEXT: 14 *)
