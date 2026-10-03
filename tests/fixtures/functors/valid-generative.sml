functor Make (X : sig end) = struct
  datatype d = D of int
  fun get (D n) = n
end
structure A = Make (struct end)
structure B = Make (struct end)
val a = A.D 1
val b = B.D 2
val c = A.get a + B.get b
val d = [a, A.D 3]
val () = print (if c = 3 andalso (case d of [A.D x, A.D y] => x = 1 andalso y = 3 | _ => false) then "valid-generative\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-generative *)
