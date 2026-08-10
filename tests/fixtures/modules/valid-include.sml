signature BASE = sig type t val x : t end
signature EXTENDED = sig
  include BASE
  val y : t
end
structure A :> EXTENDED = struct type t = int val x = 1 val y = 2 end
val a = [A.x, A.y]
signature MORE = sig include BASE val z : int end
structure B : MORE = struct type t = bool val x = true val z = 3 end
val b = B.z
val () = print (if length a = 2 andalso b = 3 then "valid-include\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-include *)
