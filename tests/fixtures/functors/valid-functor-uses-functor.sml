functor Inc (X : sig val n : int end) = struct val n = X.n + 1 end
functor Twice (X : sig val n : int end) = Inc (Inc (X))
structure T = Twice (struct val n = 1 end)
val a = T.n
structure Nested = Inc (Twice (struct val n = 10 end))
val b = Nested.n
val () = print (if a = 3 andalso b = 13 then "valid-functor-uses-functor\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-functor-uses-functor *)
