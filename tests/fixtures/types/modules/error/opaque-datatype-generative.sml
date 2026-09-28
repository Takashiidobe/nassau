signature S = sig datatype d = A | B end
structure Base = struct datatype d = A | B end
structure Sealed :> S = Base
val x = [Base.A, Sealed.A]
(* CHECK-ERR: × expected d, found d/2 *)
(* CHECK-ERR: :4:18] *)
