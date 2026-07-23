signature S = sig type t structure A : sig end sharing A = t end
(* CHECK-ERR: × cannot refine t: it is not a structure specification *)
(* CHECK-ERR: :1:48] *)
