(* nassau-ugc.11 *)
functor F () = struct datatype t=C end
structure A=F() structure B=F()
val x : A.t = B.C
(* CHECK-ERR: × expected A.t, found B.t *)
(* CHECK-ERR: :4:15] *)
