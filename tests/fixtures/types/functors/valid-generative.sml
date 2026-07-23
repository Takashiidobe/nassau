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
(* CHECK-STDOUT: val a : A.d *)
(* CHECK-STDOUT-NEXT: val b : B.d *)
(* CHECK-STDOUT-NEXT: val c : int *)
(* CHECK-STDOUT-NEXT: val d : A.d list *)
