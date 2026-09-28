signature S = sig type t val x : t end
functor F (X : S where type t = int) = struct val y = X.x + 1 end
structure A = F (struct type t = int val x = 1 end)
val a = A.y
(* CHECK-STDOUT: val a : int *)
