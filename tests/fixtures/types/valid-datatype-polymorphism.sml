datatype 'a stack = Empty | Push of 'a * 'a stack
fun push (x, s) = Push (x, s)
fun top (Push (x, _)) = x
fun size Empty = 0 | size (Push (_, s)) = 1 + size s
fun map_stack f Empty = Empty
  | map_stack f (Push (x, s)) = Push (f x, map_stack f s)
val ints = push (1, push (2, Empty))
val strs = map_stack (fn n => "n") ints
val poly = Empty
val wrapped = SOME Empty
val pair = (Empty, Empty)
val counted = size poly + size ints
(* CHECK-STDOUT: val push : 'a * 'a stack -> 'a stack *)
(* CHECK-STDOUT-NEXT: val top : 'a stack -> 'a *)
(* CHECK-STDOUT-NEXT: val size : 'a stack -> int *)
(* CHECK-STDOUT-NEXT: val map_stack : ('a -> 'b) -> 'a stack -> 'b stack *)
(* CHECK-STDOUT-NEXT: val ints : int stack *)
(* CHECK-STDOUT-NEXT: val strs : string stack *)
(* CHECK-STDOUT-NEXT: val poly : 'a stack *)
(* CHECK-STDOUT-NEXT: val wrapped : 'a stack option *)
(* CHECK-STDOUT-NEXT: val pair : 'a stack * 'b stack *)
(* CHECK-STDOUT-NEXT: val counted : int *)
(* CHECK-STDERR: warning: match nonexhaustive at 3:10 *)
(* CHECK-RUN-EXIT: 0 *)
