structure A = struct signature S = sig end end
(* CHECK-ERR: × expected a declaration; signatures can only be declared at the top level *)
(* CHECK-ERR: :1:22] *)
