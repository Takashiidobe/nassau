signature S = sig type 'a t end
signature T = S where type t = int
(* CHECK-ERR: × cannot refine t: the number of type parameters differs *)
(* CHECK-ERR: :2:15] *)
