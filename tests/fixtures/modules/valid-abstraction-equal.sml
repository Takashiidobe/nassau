signature S = sig type t val x : t end
structure A :> S = struct type t = int val x = 1 end
structure B :> S = struct type t = int val x = 2 end
val a = [A.x, A.x]
val b = [B.x]
structure Same :> S where type t = int = struct type t = int val x = 5 end
val c = Same.x + 1
val () = print (if length a = 2 andalso length b = 1 andalso c = 6 then "valid-abstraction-equal\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-abstraction-equal *)
