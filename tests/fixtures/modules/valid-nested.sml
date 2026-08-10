signature INNER = sig type t val v : t end
signature OUTER = sig
  structure A : INNER
  structure B : sig val n : int end
  val top : int
end
structure O :> OUTER = struct
  structure A = struct type t = string val v = "inner" end
  structure B = struct val n = 4 val hidden = 5 end
  val top = 6
end
val a = let val _ = O.A.v in 0 end
val b = O.B.n + O.top
structure Wrap = struct structure Inner : OUTER = O end
val c = Wrap.Inner.B.n
val () = print (if a = 0 andalso b = 10 andalso c = 4 then "valid-nested\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-nested *)
