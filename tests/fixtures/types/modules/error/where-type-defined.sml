signature S = sig type t = int end
signature T = S where type t = bool
(* CHECK-ERR: × cannot refine t: it is not an abstract type of the signature *)
(* CHECK-ERR: :2:15] *)
