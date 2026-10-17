(* SML'97 grammar, expression: infix constructor symbolic (nassau-ugc.20). *)
fun pi n = print (Int.toString n ^ "\n")
datatype t = :+: of int * int
infix :+:
val v = 1 :+: 2
val _ = pi (case v of a :+: b => a + b)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
