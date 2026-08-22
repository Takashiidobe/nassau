signature POINT = sig
  val origin : int * int
  val move : int * int -> int * int
  val poly : 'a -> 'a
end
structure P : POINT = struct
  val origin = (0, 0)
  fun move (x, y) = (x + 1, y + 1)
  fun poly x = x
  val extra = "not exported"
end
val a = P.origin
val b = P.move P.origin
val c = P.poly "x"
structure Q : sig val f : int -> int end = struct fun f x = x end
val d = Q.f 2
structure R : sig val f : int -> int end = struct fun f x = x + 1 end
val e = R.f 2
structure S : sig val v : int list end = struct val v = [] end
val f = S.v
(* CHECK-STDOUT: val a : int * int *)
(* CHECK-STDOUT-NEXT: val b : int * int *)
(* CHECK-STDOUT-NEXT: val c : string *)
(* CHECK-STDOUT-NEXT: val d : int *)
(* CHECK-STDOUT-NEXT: val e : int *)
(* CHECK-STDOUT-NEXT: val f : int list *)
(* CHECK-RUN-EXIT: 0 *)
