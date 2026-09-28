signature S = sig type t val x : t end
structure A :> S = struct type t = int val x = 1 end
structure B :> S = struct type t = int val x = 2 end
val same = [A.x, B.x]
(* CHECK-ERR: × expected A.t, found B.t *)
(* CHECK-ERR: :4:18] *)
