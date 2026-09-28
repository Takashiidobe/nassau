signature S = sig type t sharing type t = u end
(* CHECK-ERR: × cannot refine u: it is not a type specification *)
(* CHECK-ERR: :1:26] *)
