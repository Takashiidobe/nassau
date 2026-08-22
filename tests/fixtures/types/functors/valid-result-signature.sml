signature COUNTER = sig type t val zero : t val next : t -> t val count : t -> int end
functor Counter (X : sig val step : int end) :> COUNTER = struct
  type t = int
  val zero = 0
  fun next n = n + X.step
  fun count n = n
end
structure ByOne = Counter (struct val step = 1 end)
structure ByTwo = Counter (struct val step = 2 end)
val a = ByOne.count (ByOne.next ByOne.zero)
val b = ByTwo.next ByTwo.zero
functor Transparent (X : sig type t val x : t end) : sig type t val x : t end = X
structure T = Transparent (struct type t = int val x = 5 end)
val c = T.x + 1
(* CHECK-STDOUT: val a : int *)
(* CHECK-STDOUT-NEXT: val b : ByTwo.t *)
(* CHECK-STDOUT-NEXT: val c : int *)
(* CHECK-RUN-EXIT: 0 *)
