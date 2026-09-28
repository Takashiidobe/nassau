val x = let functor F (X : sig end) = struct end in 1 end
(* CHECK-ERR: × expected a declaration; functors can only be declared at the top level *)
(* CHECK-ERR: :1:13] *)
