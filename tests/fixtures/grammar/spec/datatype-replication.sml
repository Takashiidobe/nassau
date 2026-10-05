(* SML'97 grammar, specification: datatype replication. *)
fun pi n = print (Int.toString n ^ "\n")
datatype d = D of int
signature S = sig datatype t = datatype d end
structure A : S = struct datatype t = datatype d end
val _ = pi (case A.D 3 of D n => n)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
