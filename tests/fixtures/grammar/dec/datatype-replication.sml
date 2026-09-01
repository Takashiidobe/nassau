(* SML'97 grammar, declaration: datatype replication. *)
fun pi n = print (Int.toString n ^ "\n")
datatype t = X | Y of int
datatype u = datatype t
structure S = struct datatype v = V of int end
datatype w = datatype S.v
val _ = pi (case Y 3 : u of Y n => n | X => 0)
val _ = pi (case V 4 of V n => n)
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 3 *)
(* CHECK-STDOUT-NEXT: 4 *)
