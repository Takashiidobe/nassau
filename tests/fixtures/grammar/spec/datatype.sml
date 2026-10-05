(* SML'97 grammar, specification: datatype. *)
fun pi n = print (Int.toString n ^ "\n")
signature S = sig datatype 'a t = L | N of 'a * 'a t and u = U end
structure A : S = struct datatype 'a t = L | N of 'a * 'a t and u = U end
val _ = pi (case A.N (1, A.L) of A.N (x, _) => x | A.L => 0)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
