signature S = sig type t val x : t end
functor F (X : S where type t = int) = struct val y = X.x + 1 end
structure A = F (struct type t = int val x = 1 end)
val a = A.y
val () = print (if a = 2 then "valid-where-parameter\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-where-parameter *)
