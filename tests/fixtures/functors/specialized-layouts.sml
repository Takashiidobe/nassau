functor Compare (X : sig eqtype t val x : t val y : t end) = struct
  fun same () = X.x = X.y
  fun first {a, z} = a
end
structure Numbers = Compare (struct type t = int val x = 2 val y = 2 end)
structure Lists = Compare (struct type t = int list val x = [1, 2] val y = [1, 2] end)
structure Strings = Compare (struct type t = string val x = "a" ^ "b" val y = "ab" end)
val () = print (if Numbers.same () andalso Lists.same () andalso Strings.same () then "equal\n" else "wrong\n")
functor Add (X : sig type t val add : t * t -> t val a : t val b : t end) = struct
  val result = X.add (X.a, X.b)
end
structure Ints = Add (struct type t = int fun add (a : int, b) = a + b val a = 1 val b = 2 end)
structure Reals = Add (struct type t = real fun add (a : real, b) = a + b val a = 1.5 val b = 2.5 end)
val () = print (if Ints.result = 3 andalso Reals.result > 3.9 andalso Reals.result < 4.1 then "specialized\n" else "wrong\n")
functor Box (X : sig type t val value : t end) :> sig type t val make : unit -> t val get : t -> X.t end = struct
  datatype t = Box of X.t
  fun make () = Box X.value
  fun get (Box value) = value
end
structure A = Box (struct type t = int val value = 5 end)
structure B = Box (struct type t = string val value = "boxed" end)
val () = print (Int.toString (A.get (A.make ())) ^ " " ^ B.get (B.make ()) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: equal *)
(* CHECK-STDOUT-NEXT: specialized *)
(* CHECK-STDOUT-NEXT: 5 boxed *)
