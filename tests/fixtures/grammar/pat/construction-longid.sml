(* SML'97 grammar, pattern: construction longid. *)
fun pi n = print (Int.toString n ^ "\n")
structure S = struct datatype t = C of int | D end
fun f (S.C n) = n
  | f S.D = 0
val S.C m = S.C 3
val _ = pi (f (S.C 5) + f S.D + m)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 8 *)
