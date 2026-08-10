signature ORD = sig type t val le : t * t -> bool end
functor Sort (O : ORD) = struct
  fun insert (x, []) = [x]
    | insert (x, y :: ys) = if O.le (x, y) then x :: y :: ys else y :: insert (x, ys)
  fun sort xs = foldl insert [] xs
  fun min (a, b) = if O.le (a, b) then a else b
end
structure IntSort = Sort (struct type t = int fun le (a : int, b) = a <= b end)
structure StringSort = Sort (struct type t = string fun le (a : string, b) = a <= b end)
val a = IntSort.sort [3, 1, 2]
val b = StringSort.min ("b", "a")
val c = IntSort.min (1, 2) + 1
val () = print (if a = [1, 2, 3] andalso b = "a" andalso c = 2 then "valid-functors\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-functors *)
