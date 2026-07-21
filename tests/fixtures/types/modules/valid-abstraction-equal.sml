signature S = sig type t val x : t end
structure A :> S = struct type t = int val x = 1 end
structure B :> S = struct type t = int val x = 2 end
val a = [A.x, A.x]
val b = [B.x]
structure Same :> S where type t = int = struct type t = int val x = 5 end
val c = Same.x + 1
(* CHECK-STDOUT: val a : A.t list *)
(* CHECK-STDOUT-NEXT: val b : B.t list *)
(* CHECK-STDOUT-NEXT: val c : int *)
