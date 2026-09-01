(* SML'97 grammar, specification: sharing type longid (nassau-ugc.19). *)
fun pi n = print (Int.toString n ^ "\n")
signature T = sig type t end
signature S = sig structure A : T structure B : T sharing type A.t = B.t val f : A.t -> B.t end
structure M : S = struct structure A = struct type t = int end structure B = A fun f x = x end
val _ = pi (M.f 2)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 2 *)
