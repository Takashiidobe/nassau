structure M = struct
  datatype d = A | B of int
  val v = B 1
  fun value (B n) = n | value A = 0
  structure N = struct val w = 2 end
end
val a = let open M in value v end
val b = let open M.N in w + 1 end
local
  open M
in
  val c = value (B 5)
  val d = A
end
structure O = struct open M val extra = N.w end
val e = O.extra
val f = O.v
val () = print (if a = 1 andalso b = 3 andalso c = 5 andalso (case d of M.A => true | _ => false) andalso e = 2 andalso O.value f = 1 then "valid-open\n" else "wrong\n")
(* CHECK-EXIT: 0 *)
(* CHECK-STDOUT: valid-open *)
