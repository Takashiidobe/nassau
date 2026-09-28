functor Make (X : sig end) = struct datatype d = D end
structure A = Make (struct end)
structure B = Make (struct end)
val x = [A.D, B.D]
(* CHECK-ERR: × expected A.d, found B.d *)
(* CHECK-ERR: :4:15] *)
