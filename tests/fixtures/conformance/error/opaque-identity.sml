(* nassau-ugc.11 *)
structure A :> sig type t val x:t end = struct type t=int val x=1 end
val x:int = A.x
(* CHECK-ERR: × expected int, found A.t *)
(* CHECK-ERR: :3:13] *)
