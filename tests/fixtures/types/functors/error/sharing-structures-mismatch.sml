signature S = sig
  structure A : sig type t end
  structure B : sig type t end
  sharing A = B
end
functor F (X : S) = struct end
structure R = F (struct structure A = struct type t = int end structure B = struct type t = string end end)
(* CHECK-ERR: × type t does not match its specification: the definition differs from the *)
(* CHECK-ERR: :7:18] *)
