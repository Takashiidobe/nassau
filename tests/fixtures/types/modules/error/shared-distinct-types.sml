signature T = sig type t end
signature S = sig structure A:T structure B:T sharing type B.t=A.t end
structure Bad:S = struct
 structure A=struct type t=int end
 structure B=struct type t=bool end
end
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :3:19] *)
