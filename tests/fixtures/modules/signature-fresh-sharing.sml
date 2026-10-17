signature T = sig eqtype t val x:t end
signature Independent = sig structure A:T structure B:T end
structure I : Independent = struct
  structure A = struct type t=int val x=1 end
  structure B = struct type t=bool val x=true end
end
val _ = print (Int.toString I.A.x ^ Bool.toString I.B.x ^ "\n")
signature Shared = sig structure A:T structure B:T sharing type B.t = A.t end
structure S : Shared = struct
 structure A = struct type t=int val x=3 end
 structure B = A
end
val _ = print (Int.toString (S.A.x + S.B.x) ^ "\n")
signature Nested = sig structure L:Shared structure R:Shared sharing L=R end
structure N :> Nested = struct structure L=S structure R=L end
val _ = print (Bool.toString (N.L.A.x = N.R.B.x) ^ "\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: 1true *)
(* CHECK-STDOUT-NEXT: 6 *)
(* CHECK-STDOUT-NEXT: true *)
