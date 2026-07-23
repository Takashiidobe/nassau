functor Wrap (X : sig type t val x : t val show : t -> string end) = struct
  val shown = X.show X.x
  val pair = (X.x, X.x)
end
structure W = Wrap (struct type t = int val x = 3 fun show (n : int) = "int" end)
val a = W.shown
val b = W.pair
structure V = Wrap (struct type t = bool val x = true fun show (b : bool) = "bool" end)
val c = V.pair
(* CHECK-STDOUT: val a : string *)
(* CHECK-STDOUT-NEXT: val b : int * int *)
(* CHECK-STDOUT-NEXT: val c : bool * bool *)
