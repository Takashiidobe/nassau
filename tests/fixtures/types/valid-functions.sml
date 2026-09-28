fun fact 0 = 1
  | fact n = n * fact (n - 1)
fun len [] = 0
  | len (_ :: rest) = 1 + len rest
fun app f [] = []
  | app f (x :: xs) = f x :: app f xs
fun zip (a :: xs, b :: ys) = (a, b) :: zip (xs, ys)
  | zip _ = []
fun even 0 = true
  | even n = odd (n - 1)
and odd 0 = false
  | odd n = even (n - 1)
val rec loop = fn 0 => "done" | n => loop (n - 1)
val rec ones = fn n => if n = 0 then [] else 1 :: ones (n - 1)
fun curried a b c = (a, b, c)
fun typed (x : int) : int = x
fun annotated (f : 'a -> 'b) (x : 'a) : 'b = f x
fun fib n = if n < 2 then n else fib (n - 1) + fib (n - 2)
val total = fact 5 + len [1, 2]
(* CHECK-STDOUT: val fact : int -> int *)
(* CHECK-STDOUT-NEXT: val len : 'a list -> int *)
(* CHECK-STDOUT-NEXT: val app : ('a -> 'b) -> 'a list -> 'b list *)
(* CHECK-STDOUT-NEXT: val zip : 'a list * 'b list -> ('a * 'b) list *)
(* CHECK-STDOUT-NEXT: val even : int -> bool *)
(* CHECK-STDOUT-NEXT: val odd : int -> bool *)
(* CHECK-STDOUT-NEXT: val loop : int -> string *)
(* CHECK-STDOUT-NEXT: val ones : int -> int list *)
(* CHECK-STDOUT-NEXT: val curried : 'a -> 'b -> 'c -> 'a * 'b * 'c *)
(* CHECK-STDOUT-NEXT: val typed : int -> int *)
(* CHECK-STDOUT-NEXT: val annotated : ('a -> 'b) -> 'a -> 'b *)
(* CHECK-STDOUT-NEXT: val fib : int -> int *)
(* CHECK-STDOUT-NEXT: val total : int *)
