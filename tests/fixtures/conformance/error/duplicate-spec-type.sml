signature S = sig type t type t end
(* CHECK-ERR: × duplicate type name 't' in signature *)
(* CHECK-ERR: :1:26] *)
