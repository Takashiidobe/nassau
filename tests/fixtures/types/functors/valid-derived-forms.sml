functor Pair (type t val x : t val y : t) = struct
  val both = [x, y]
  val first = x
end
structure P = Pair (type t = int val x = 1 val y = 2)
val a = P.both
val b = P.first + 1
structure Q = Pair (type t = string; val x = "x"; val y = "y")
val c = Q.both
functor Unit () = struct val one = 1 end
structure U = Unit ()
val d = U.one
(* CHECK-STDOUT: val a : int list *)
(* CHECK-STDOUT-NEXT: val b : int *)
(* CHECK-STDOUT-NEXT: val c : string list *)
(* CHECK-STDOUT-NEXT: val d : int *)
(* CHECK-RUN-EXIT: 0 *)
