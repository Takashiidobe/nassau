signature SHARED = sig
  type t
  type u
  sharing type t = u
  val a : t
  val b : u
end
structure A :> SHARED = struct type t = int type u = int val a = 1 val b = 2 end
val x = length [A.a, A.b]
signature TWO = sig
  structure X : sig type t val v : t end
  structure Y : sig type t val w : t end
  sharing type X.t = Y.t
end
structure B :> TWO = struct
  structure X = struct type t = int val v = 1 end
  structure Y = struct type t = int val w = 2 end
end
val y = length [B.X.v, B.Y.w]
(* CHECK-STDOUT: val x : int *)
(* CHECK-STDOUT-NEXT: val y : int *)
