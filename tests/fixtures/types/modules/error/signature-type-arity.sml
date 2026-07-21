signature S = sig type t val x : int t end
(* CHECK-ERR: × type 't' expects 0 type argument(s), found 1 *)
(* CHECK-ERR: :1:34] *)
