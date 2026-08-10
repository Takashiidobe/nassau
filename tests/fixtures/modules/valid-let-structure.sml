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
val () = print (if a = 11 andalso b = 12 andalso c = "q" then "valid-let-structure\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-let-structure *)
