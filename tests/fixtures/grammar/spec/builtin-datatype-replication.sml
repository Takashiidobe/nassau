signature BUILTIN =
sig
  datatype order = datatype order
  datatype list = datatype list
end
structure B :> BUILTIN =
struct
  datatype order = datatype order
  datatype list = datatype list
end
val xs : int B.list = [1, 2]
val order : order = B.LESS
val _ = print (case (order, xs) of (B.LESS, x :: _) => Int.toString x ^ "\n" | _ => "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1 *)
