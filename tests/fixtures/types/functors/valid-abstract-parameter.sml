functor Use (X : sig type t val make : int -> t val get : t -> int end) = struct
  val value = X.get (X.make 4)
end
structure Impl :> sig type t val make : int -> t val get : t -> int end = struct
  type t = int
  fun make n = n
  fun get n = n
end
structure U = Use (Impl)
val a = U.value + 1
(* CHECK-STDOUT: val a : int *)
