structure S = let
  val base = 10
  fun add n = n + base
in
  struct val v = add 1 fun bump n = add n end
end
val a = S.v
val b = S.bump 2
structure T = let structure I = struct val q = "q" end in I end
val c = T.q
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : string *)
(* CHECK-RUN-EXIT: 0 *)
