functor Inc (X : sig val n : int end) = struct val n = X.n + 1 end
functor Twice (X : sig val n : int end) = Inc (Inc (X))
structure T = Twice (struct val n = 1 end)
val a = T.n
structure Nested = Inc (Twice (struct val n = 10 end))
val b = Nested.n
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-RUN-EXIT: 0 *)
