functor Wrap (X : sig type t val x : t val show : t -> string end) = struct
  val shown = X.show X.x
  val pair = (X.x, X.x)
end
structure W = Wrap (struct type t = int val x = 3 fun show (n : int) = "int" end)
val a = W.shown
val b = W.pair
structure V = Wrap (struct type t = bool val x = true fun show (b : bool) = "bool" end)
val c = V.pair
val () = print (if a = "int" andalso b = (3, 3) andalso c = (true, true) then "valid-parameter-types\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-parameter-types *)
