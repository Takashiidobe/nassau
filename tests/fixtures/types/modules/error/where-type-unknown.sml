signature S = sig type t end
signature T = S where type u = int
(* CHECK-ERR: × cannot refine u: it is not an abstract type of the signature *)
(* CHECK-ERR: :2:15] *)
