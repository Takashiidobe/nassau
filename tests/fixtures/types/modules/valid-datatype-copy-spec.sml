structure A = struct datatype t = X | Y of int end
signature S = sig datatype u = datatype A.t val v : u end
structure B : S = struct datatype u = datatype A.t val v = A.Y 1 end
val a = B.v
val b = [B.v, A.X, B.X]
(* CHECK-STDOUT: val a : A.t *)
(* CHECK-STDOUT-NEXT: val b : A.t list *)
(* CHECK-RUN-EXIT: 0 *)
