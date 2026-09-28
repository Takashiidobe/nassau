signature S = sig type t type u sharing type t = u end
structure A : S = struct type t = int type u = string end
(* CHECK-ERR: × type u does not match its specification: the definition differs from the *)
(* CHECK-ERR: :2:19] *)
