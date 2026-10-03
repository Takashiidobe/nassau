signature STACK = sig
  type 'a t
  val empty : 'a t
  val push : 'a * 'a t -> 'a t
  val pop : 'a t -> ('a * 'a t) option
  val size : 'a t -> int
end
structure Stack :> STACK = struct
  type 'a t = 'a list
  val empty = []
  fun push (x, s) = x :: s
  fun pop [] = NONE
    | pop (x :: s) = SOME (x, s)
  val size = length
end
val a = Stack.push (1, Stack.empty)
val b = Stack.pop a
val c = Stack.size (Stack.push ("s", Stack.empty))
val d = Stack.empty
structure Counter :> sig type t val zero : t val next : t -> t val show : t -> string end =
struct
  type t = int
  val zero = 0
  fun next n = n + 1
  fun show n = "n"
end
val e = Counter.show (Counter.next Counter.zero)
(* CHECK-STDOUT: val a : int Stack.t *)
(* CHECK-STDOUT-NEXT: val b : (int * int Stack.t) option *)
(* CHECK-STDOUT-NEXT: val c : int *)
(* CHECK-STDOUT-NEXT: val d : 'a Stack.t *)
(* CHECK-STDOUT-NEXT: val e : string *)
(* CHECK-RUN-EXIT: 0 *)
