(* SML'97 grammar, expression: infix constructor alphanumeric (nassau-ugc.20). *)
fun pi n = print (Int.toString n ^ "\n")
datatype t = Pair of int * int
infix Pair
val v = 1 Pair 2
val _ = pi (case v of a Pair b => a + b)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
