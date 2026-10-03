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
val _ = print (if c = 3 andalso A.get a = 1 andalso B.get b = 2 andalso map A.get d = [1,3] then "functors verified\n" else raise Fail "functors")
(* CHECK-STDOUT: val a : A.d *)
(* CHECK-STDOUT-NEXT: val b : B.d *)
(* CHECK-STDOUT-NEXT: val c : int *)
(* CHECK-STDOUT-NEXT: val d : A.d list *)
(* CHECK-RUN-EXIT: 0 *)
(* CHECK-RUN-STDOUT: functors verified *)
