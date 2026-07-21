val x = let signature S = sig end in 1 end
(* CHECK-ERR: × expected a declaration; signatures can only be declared at the top level *)
(* CHECK-ERR: :1:13] *)
