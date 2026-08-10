structure A = struct datatype t = X | Y of int end
signature S = sig datatype u = datatype A.t val v : u end
structure B : S = struct datatype u = datatype A.t val v = A.Y 1 end
val a = B.v
val b = [B.v, A.X, B.X]
val () = print (if (case a of A.Y n => n = 1 | _ => false) andalso length b = 3 then "valid-datatype-copy-spec\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-datatype-copy-spec *)
