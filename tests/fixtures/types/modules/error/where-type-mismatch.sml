signature S = sig type t val x : t end
structure A :> S where type t = int = struct type t = string val x = "s" end
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :2:39] *)
