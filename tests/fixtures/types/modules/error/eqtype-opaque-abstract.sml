structure S :> sig type t val a : t val b : t end = struct type t = int val a = 1 val b = 2 end
val x = S.a = S.b
(* CHECK-ERR: × type S.t does not admit equality *)
(* CHECK-ERR: :2:9] *)
