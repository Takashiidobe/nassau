signature PAIR = sig
  structure A : sig type t val x : t end
  structure B : sig type t val y : t end
  sharing A = B
end
functor Join (P : PAIR) = struct val both = [P.A.x, P.B.y] end
structure J = Join (struct
  structure A = struct type t = int val x = 1 end
  structure B = struct type t = int val y = 2 end
end)
val a = J.both
(* CHECK-STDOUT: val a : int list *)
