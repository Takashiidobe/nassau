(* nassau-ugc.11 *)
signature S = sig datatype t = C end
structure A:S = struct type t = int val C = 1 end
(* CHECK-ERR: × the structure does not provide datatype t, which the signature specifies *)
(* CHECK-ERR: :3:17] *)
