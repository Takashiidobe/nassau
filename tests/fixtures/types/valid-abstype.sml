abstype counter = Counter of int
with
  val zero = Counter 0
  fun bump (Counter n) = Counter (n + 1)
  fun read (Counter n) = n
end
abstype 'a box = Box of 'a
with
  fun box x = Box x
  fun unbox (Box x) = x
end
val two = read (bump (bump zero))
val boxed = box "s"
val out = unbox boxed
(* CHECK-STDOUT: val zero : counter *)
(* CHECK-STDOUT-NEXT: val bump : counter -> counter *)
(* CHECK-STDOUT-NEXT: val read : counter -> int *)
(* CHECK-STDOUT-NEXT: val box : 'a -> 'a box *)
(* CHECK-STDOUT-NEXT: val unbox : 'a box -> 'a *)
(* CHECK-STDOUT-NEXT: val two : int *)
(* CHECK-STDOUT-NEXT: val boxed : string box *)
(* CHECK-STDOUT-NEXT: val out : string *)
(* CHECK-RUN-EXIT: 0 *)
