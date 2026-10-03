signature S = sig type 'a t end
signature T = S where type t = int
(* POLYML-SKIP: Poly/ML 5.9.2 accepts where-type refinements with mismatched arity *)
(* CHECK-ERR: × cannot refine t: the number of type parameters differs *)
(* CHECK-ERR: :2:15] *)
