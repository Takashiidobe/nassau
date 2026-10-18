signature S = sig datatype t = ref of int end
(* CHECK-ERR: × 'ref' cannot be used as a binding name *)
(* CHECK-ERR: :1:32] *)
